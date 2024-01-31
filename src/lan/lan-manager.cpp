#include "lan-manager.hpp"
#include "lan-packet.hpp"
#define LOG_LVL LOG_LVL_DEBUG
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

Channel* Manager::open()
{
    return open(sf::IpAddress::Any, 0);
}

Channel* Manager::open(sf::IpAddress addr, uint16_t port)
{
    _channels[addr.toInteger()].set_addr(addr);
    _channels[addr.toInteger()].set_port(port);
    log_debug("Open channel ", addr.toString(), ":", port);
    return &_channels[addr.toInteger()];
}

void Manager::close(Channel *channel)
{
    _channels.erase(channel->_addr.toInteger());
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
            auto it = _channels.find(client_addr.value().toInteger());
            if (it != _channels.end()) {
                it->second.take_packet(&packet, client_addr.value(), client_port);
            } else {
                auto it = _channels.find(sf::IpAddress::Any.toInteger());
                if (it != _channels.end()) {
                    it->second.take_packet(&packet, client_addr.value(), client_port);
                }
            }
        }
    }

// send ////////////////////////////////////////////////////////////////////////////////////////////

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
            log_debug("Send to ", c.second._addr.toString(), ":", c.second._port,
                      " len: ", c.second.get_packet()->getDataSize(),
                      " tag: ", (int)c.second.get_packet()->get_tag(),
                      " sequence counter: ", c.second.get_packet()->get_sequence_counter());
            c.second.pop_packet();
        }
    }

////////////////////////////////////////////////////////////////////////////////////////////////////
}
