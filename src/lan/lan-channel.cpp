#include "lan-channel.hpp"
#define LOG_LVL LOG_LVL_DEBUG
#include "logger.hpp"

#include <cstring>

using namespace Lan;

Channel::Channel() :
    _addr(sf::IpAddress::Any),
    _port(sf::UdpSocket::AnyPort),
    _send_sequence_counter(0),
    _send_important_sequence_counter(0),
    _recv_sequence_counter(0),
    _recv_important_sequence_counter(0),
    _waiting_ack(false)
{

}

Status Channel::send(const Packet &packet)
{
    if (packet.is_important()) {
        return send_important(packet);
    }
    return send_not_important(packet);
}

Status Channel::send_important(const struct Packet &packet)
{
    if (_send_important_buff.size() >= packet_buffer_max_len) {
        return Status::NOT_READY;
    }
    _send_important_buff.push(packet);
    _send_important_sequence_counter++;
    _send_important_buff.back().set_sequence_counter(_send_important_sequence_counter);
    return Status::OK;
}

Status Channel::send_not_important(const struct Packet &packet)
{
    if (_send_buff.size() >= packet_buffer_max_len) {
        return Status::NOT_READY;
    }
    _send_buff.push(packet);
    _send_sequence_counter++;
    _send_buff.back().set_sequence_counter(_send_sequence_counter);
    return Status::OK;
}

Status Channel::recv(Packet &packet)
{
    if (_recv_buff.empty()) {
        return Status::NOT_READY;
    }
    packet = _recv_buff.front();
    _recv_buff.pop();
    return Status::OK;
}

void Channel::set_addr(const sf::IpAddress &addr)
{
    _addr = addr;
}

void Channel::set_port(uint16_t port)
{
    _port = port;
}

const sf::IpAddress& Channel::get_addr()
{
    if (_addr == sf::IpAddress::Any && _send_buff.front().get_tag() == Packet::CHANNEL_TAG_ACK) {
        return _send_buff.front().get_sender_addr();
    }
    return _addr;
}

uint16_t Channel::get_port()
{
    if (_addr == sf::IpAddress::Any && _send_buff.front().get_tag() == Packet::CHANNEL_TAG_ACK) {
        return _send_buff.front().get_sender_port();
    }
    return _port;
}

bool Channel::has_packet_to_send()
{
    return (!_send_important_buff.empty() && !_waiting_ack) || !_send_buff.empty();
}

Packet *Channel::get_packet()
{
    if (!_send_important_buff.empty() && !_waiting_ack) {
        return &_send_important_buff.front();
    }
    return &_send_buff.front();
}

void Channel::pop_packet()
{
    if (!_send_important_buff.empty() && !_waiting_ack) {
        _waiting_ack = true;
        log_debug("Waiting ACK ", _send_important_buff.front().get_sequence_counter());
    } else {
        _send_buff.pop();
    }
}

Status Channel::take_packet(Packet *packet, sf::IpAddress addr, uint16_t port)
{
    if (!packet->is_important()) {
        int sequence_diff = ((int64_t)_recv_sequence_counter + 1) - packet->get_sequence_counter();
        if (sequence_diff < 0) {
            log_warn("Sequence jump forward by ", -sequence_diff ," detected");
        }
        _recv_sequence_counter = packet->get_sequence_counter();
    }

    if (packet->get_tag() == Packet::CHANNEL_TAG_ACK) {
        log_debug("Recv ACK for ", packet->get_sequence_counter());
        uint32_t ack_id;
        *packet >> ack_id;
        if (_waiting_ack && _send_important_buff.front().get_sequence_counter() == ack_id) {
            log_debug("Accept ACK");
            _send_important_buff.pop();
            _waiting_ack = false;
        } else if (_waiting_ack && _send_important_buff.front().get_sequence_counter() < ack_id) {
            log_warn("Received ack on a package that has not yet been sent");
        }
        return Status::OK;
    }

    if (packet->is_important()) {
        Packet ansv(Packet::CHANNEL_TAG_ACK);
        ansv << packet->get_sequence_counter();
        ansv.set_sender_addr(addr);
        ansv.set_sender_port(port);
        log_debug("Send ACK for ", packet->get_sequence_counter());
        if (send(ansv) != Status::OK) {
            log_error("Failed to send ACK on packet: ", packet->get_sequence_counter());
        }
        if (packet->get_sequence_counter() <= _recv_important_sequence_counter)
        {
            log_debug("Drop old important packet ", packet->get_sequence_counter(),
                      " current ", _recv_important_sequence_counter);
            return Status::OK; // drop old packet
        }
        _recv_important_sequence_counter = packet->get_sequence_counter();
    }

    if (_recv_buff.size() >= packet_buffer_max_len) {
        log_warn("Recv buffer is full. Drop packet from ", addr.toString(), ":", port);
        if (packet->is_important()) {
            log_error("Droped important packet");
        }
        return Status::NOT_READY;
    }

    packet->set_sender_addr(addr);
    packet->set_sender_port(port);
    _recv_buff.push(*packet);
    return Status::OK;
}
