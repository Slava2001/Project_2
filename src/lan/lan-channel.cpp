#include "lan-channel.hpp"
#define LOG_LVL LOG_LVL_INFO
#include "logger.hpp"

#include <cstring>

using namespace Lan;

Channel::Channel():
    _addr(sf::IpAddress::Any),
    _port(sf::UdpSocket::AnyPort),
    _send_sequence_counter(0),
    _send_important_sequence_counter(0),
    _recv_sequence_counter(0),
    _recv_important_sequence_counter(0),
    _waiting_ack(false),
    _resend_count(0)
{

}

void Channel::reset()
{
    _status = Status::OK;
    _send_sequence_counter = 0;
    _send_important_sequence_counter = 0;
    _recv_sequence_counter = 0;
    _recv_important_sequence_counter = 0;
    _waiting_ack = false;
    _waiting_time.reset();
    _resend_count = 0;
    _time_since_last_receipt.reset();
    _send_buff = std::queue<Lan::Packet>();
    _send_important_buff = std::queue<Lan::Packet>();
    _recv_buff= std::queue<Lan::Packet>();
}

Status Channel::send(const Packet &packet)
{
    if (_status != Status::OK) {
        return _status;
    }
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
    _send_important_buff.back().set_sequence_counter(_send_important_sequence_counter);
    _send_important_sequence_counter++;
    return Status::OK;
}

Status Channel::send_not_important(const struct Packet &packet)
{
    if (_send_buff.size() >= packet_buffer_max_len) {
        return Status::NOT_READY;
    }
    _send_buff.push(packet);
    _send_buff.back().set_sequence_counter(_send_sequence_counter);
    _send_sequence_counter++;
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

const sf::IpAddress& Channel::get_addr() const
{
    if (_addr == sf::IpAddress::Any && _send_buff.front().get_tag() == Packet::CHANNEL_TAG_ACK) {
        return _send_buff.front().get_sender_addr();
    }
    return _addr;
}

uint16_t Channel::get_port() const
{
    if (_addr == sf::IpAddress::Any && _send_buff.front().get_tag() == Packet::CHANNEL_TAG_ACK) {
        return _send_buff.front().get_sender_port();
    }
    return _port;
}

bool Channel::has_packet_to_send()
{
    if (_status != Status::OK) {
        return false;
    }
    if (_time_since_last_receipt.getElapsedTime() >= time_between_pings) {
        _time_since_last_receipt.restart();
        Packet ping(Packet::CHANNEL_TAG_PING);
        if (send_important(ping) != Status::OK) {
            log_error("Failed to send PING to ", _addr.toString(), ":", _port);
        }
        log_debug("Send PING to ", _addr.toString(), ":", _port);
    }
    if (_waiting_time.isRunning() && _waiting_time.getElapsedTime() > time_between_resends) {
        _resend_count++;
        if (_resend_count > resends_count) {
            log_warn("Re-send timeout. addr: ", _addr.toString(), ":", _port);
            _status = Status::TIMEOUT;
            return false;
        }
        _waiting_ack = false;
    }
    return (!_send_important_buff.empty() && !_waiting_ack) || !_send_buff.empty();
}

Packet* Channel::get_packet()
{
    if (!_send_important_buff.empty() && !_waiting_ack) {
        return &_send_important_buff.front();
    }
    return &_send_buff.front();
}

void Channel::pop_packet()
{
    if (!_send_important_buff.empty() && !_waiting_ack) {
        _waiting_time.start();
        _waiting_ack = true;
        log_debug("Waiting ACK ", _send_important_buff.front().get_sequence_counter());
    } else {
        _send_buff.pop();
    }
}

Status Channel::take_packet(Packet *packet, sf::IpAddress addr, uint16_t port)
{
    _time_since_last_receipt.restart();
    packet->set_sender_addr(addr);
    packet->set_sender_port(port);

    log_debug("Take packet counter: ", packet->get_sequence_counter(),
              " recv counter: ", _recv_sequence_counter,
              " recv important counter: ", _recv_important_sequence_counter,
              " is important: ", (packet->is_important()? "true": "false"));
    if (!packet->is_important()) {
        bool sender_channel_reset = (packet->get_sequence_counter() == 0);
        if (!sender_channel_reset) {
            int sequence_diff = ((int64_t)_recv_sequence_counter + 1) - packet->get_sequence_counter();
            if (sequence_diff > 0) {
                log_error("Sequence jump back by ", sequence_diff ," detected");
            } else if (sequence_diff < 0) {
                log_warn("Sequence jump forward by ", -sequence_diff ," detected");
            }
        }
        _recv_sequence_counter = packet->get_sequence_counter();
    }

    if (packet->get_tag() == Packet::CHANNEL_TAG_ACK) {
        uint32_t ack_id;
        *packet >> ack_id;
        log_debug("Recv ACK for ", ack_id);
        if (_waiting_ack && _send_important_buff.front().get_sequence_counter() == ack_id) {
            log_debug("Accept ACK");
            _send_important_buff.pop();
            _waiting_ack = false;
            _waiting_time.reset();
        } else if (_waiting_ack && _send_important_buff.front().get_sequence_counter() < ack_id) {
            log_warn("Received ack on a package that has not yet been sent");
        }
        return Status::OK;
    }

    if (packet->is_important()) {
        send_ack(packet);
        bool sender_channel_reset = (packet->get_sequence_counter() == 0);
        if (!sender_channel_reset &&
            packet->get_sequence_counter() <= _recv_important_sequence_counter)
        {
            log_debug("Drop old important packet ", packet->get_sequence_counter(),
                      " current ", _recv_important_sequence_counter);
            return Status::OK; // drop old packet
        }
        _recv_important_sequence_counter = packet->get_sequence_counter();
    }

    if (packet->get_tag() == Packet::CHANNEL_TAG_PING) {
        // drop channel packets
        return Status::OK;
    }

    if (_recv_buff.size() >= packet_buffer_max_len) {
        log_warn("Recv buffer is full. Drop packet from ", addr.toString(), ":", port);
        if (packet->is_important()) {
            log_error("Droped important packet");
        }
        return Status::NOT_READY;
    }

    _recv_buff.push(*packet);
    return Status::OK;
}
