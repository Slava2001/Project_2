#include "lan-channel.hpp"
#define LOG_LVL LOG_LVL_INFO
#include "logger.hpp"

#include <cstring>

using namespace Lan;

Packet::Packet() : _addr(sf::IpAddress::Any)
{
}

void Packet::set_important(bool flag)
{
    _is_important = flag;
}

void Packet::set_tag(uint8_t tag)
{
    _tag = tag & 0b01111111;
}

bool Packet::is_important() const
{
    return _is_important;
}

uint8_t Packet::get_tag() const
{
    return _tag;
}

uint32_t Packet::get_sequence_counter() const
{
    return _sequence_counter;
}

const sf::IpAddress Packet::get_sender_addr() const
{
    return _addr;
}

uint16_t Packet::get_sender_port() const
{
    return _port;
}

void Packet::set_sequence_counter(uint32_t counter)
{
    _sequence_counter = counter;
}

void Packet::set_sender_addr(const sf::IpAddress &addr)
{
    _addr = addr;
}

void Packet::set_sender_port(uint16_t port)
{
    _port = port;
}

const void* Packet::onSend(std::size_t& size)
{
    tmp.clear();
    uint8_t tag_and_value = _tag | (_is_important? 0x80: 0);
    uint32_t sequence_counter = _sequence_counter;
    tmp << tag_and_value;
    tmp << sequence_counter;
    tmp.append(getData(), getDataSize());
    size = tmp.getDataSize();
    return tmp.getData();
}

void Packet::onReceive(const void* data, std::size_t size)
{
    if (size < _header_size) {
        return;
    }

    tmp.clear();
    tmp.append(data, _header_size);
    uint8_t tag_and_value;
    tmp >> tag_and_value;
    _is_important = tag_and_value & 0b10000000;
    _tag = tag_and_value & 0b01111111;
    tmp >> _sequence_counter;
    append((uint8_t *)data + _header_size, size - _header_size);
}


Channel::Channel() :
    _addr(sf::IpAddress::Any),
    _port(0),
    _send_sequence_counter(0),
    _recv_sequence_counter(0)
{

}

Status Channel::send(const Packet &packet)
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

bool Channel::has_packet_to_send()
{
    return !_send_buff.empty();
}

Packet *Channel::get_packet()
{
    return &_send_buff.front();
}

void Channel::pop_packet()
{
    _send_buff.pop();
}

Status Channel::take_packet(Packet *packet, sf::IpAddress addr, uint16_t port)
{
    int sequence_diff = ((int64_t)_recv_sequence_counter + 1) - packet->get_sequence_counter();
    if (sequence_diff > 0) {
        log_error("Sequence jump back by ", sequence_diff ," detected");
    } else if (sequence_diff < 0) {
        log_warn("Sequence jump forward by ", -sequence_diff ," detected");
    }
    _recv_sequence_counter = packet->get_sequence_counter();

    if (_recv_buff.size() >= packet_buffer_max_len) {
        log_warn("Recv buffer is full. Drop packet from ", addr.toString(), ":", port);
        return Status::NOT_READY;
    }

    packet->set_sender_addr(addr);
    packet->set_sender_port(port);
    _recv_buff.push(*packet);
    return Status::OK;
}
