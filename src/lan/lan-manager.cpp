#include "lan-manager.hpp"
#include "lan-packet.hpp"
#define LOG_LVL LOG_LVL_INFO
#include "logger.hpp"

#include <cstring>

using namespace Lan;

Manager::Manager() : _is_started(false)
{
}

void Manager::start(uint16_t port)
{
    if (_socket.bind(port) != sf::Socket::Status::Done) {
        log_fatal("Failed to bind socket to port: ", port);
        throw std::runtime_error("Failed to bind socket");
    }
    _socket.setBlocking(false);
    _is_started = true;
    log_info("Server started. Port: ", get_port());
}

uint16_t Manager::get_port()
{
    return _socket.getLocalPort();
}

void Manager::stop()
{
}

Recv_channel* Manager::get_default_channel()
{
    return &_default_channel;
}

uint64_t Manager::get_addr_hash(const sf::IpAddress &addr, uint16_t port)
{
    return ((((uint64_t)addr.toInteger()) << 32) | port);
}

Channel* Manager::open(sf::IpAddress addr, uint16_t port)
{
    Channel* channel = &_channels[get_addr_hash(addr, port)];
    channel->set_addr(addr);
    channel->set_port(port);
    log_debug("Open channel ", addr.toString(), ":", port);
    return channel;
}

void Manager::close(Channel *channel)
{
    if (channel) {
        _channels.erase(get_addr_hash(channel->get_addr(), channel->get_port()));
    }
}

void Manager::update()
{
    if (!_is_started) {
        return;
    }
// recv ////////////////////////////////////////////////////////////////////////////////////////////

    Packet packet;
    std::optional<sf::IpAddress> client_addr;
    uint16_t client_port;

    sf::Socket::Status status = _socket.receive(packet, client_addr, client_port);
    if (status == sf::Socket::Status::Error) {
        log_fatal("Failed to receive data. Socket error");
        throw std::runtime_error("Failed to receive data");
    }

    if (status == sf::Socket::Status::Done) {
        log_debug("Recv from ", client_addr.value().toString(), ":", client_port,
                  " len: ", packet.getDataSize(),
                  " tag: ", (int)packet.get_tag(),
                  " sequence counter: ", packet.get_sequence_counter());
        if (!client_addr.has_value()) {
            log_warn("Client has not addres");
        } else {
            auto it = _channels.find(get_addr_hash(client_addr.value(), client_port));
            if (it != _channels.end()) {
                it->second.take_packet(&packet, client_addr.value(), client_port);
            } else {
                _default_channel.take_packet(&packet, client_addr.value(), client_port);
            }
        }
    }

// send ////////////////////////////////////////////////////////////////////////////////////////////

    if (_default_channel.has_packet_to_send()) {
        sf::Socket::Status status = _socket.send(*_default_channel.get_packet(),
                                                 _default_channel.get_addr(),
                                                 _default_channel.get_port());
        if (status == sf::Socket::Status::Error) {
            log_fatal("Failed to send ACK to ", _default_channel.get_addr().toString(), ":",
                                                _default_channel.get_port());
            throw std::runtime_error("Failed to send ACK");
        }

        if (status == sf::Socket::Status::Done) {
            log_debug("Send ACK to ", _default_channel.get_addr().toString(), ":",
                                      _default_channel.get_port());
            _default_channel.pop_packet();
        }
    }

    for (auto &c: _channels) {
        if (!c.second.has_packet_to_send()) {
            continue;
        }

        sf::Socket::Status status = _socket.send(*c.second.get_packet(), c.second.get_addr(),
                                                 c.second.get_port());
        if (status == sf::Socket::Status::Error) {
            log_fatal("Failed to send data to ", c.second.get_addr().toString(), ":",
                                                 c.second.get_port());
            throw std::runtime_error("Failed to send data");
        }

        if (status == sf::Socket::Status::Done) {
            log_debug("Send to ", c.second.get_addr().toString(), ":", c.second.get_port(),
                      " len: ", c.second.get_packet()->getDataSize(),
                      " tag: ", (int)c.second.get_packet()->get_tag(),
                      " sequence counter: ", c.second.get_packet()->get_sequence_counter());
            c.second.pop_packet();
        }
    }

////////////////////////////////////////////////////////////////////////////////////////////////////
}
