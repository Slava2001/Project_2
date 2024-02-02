#include "lan-packet.hpp"
#define LOG_LVL LOG_LVL_INFO
#include "logger.hpp"

#include <cstring>

using namespace Lan;

Packet::Packet():
    _tag(Packet::TAG_UNDEFINED),
    _sequence_counter(0),
    _addr(sf::IpAddress::Any),
    _port(0)
{
}

Packet::Packet(Packet::Tag tag):
    _tag(tag),
    _sequence_counter(0),
    _addr(sf::IpAddress::Any),
    _port(0)
{
}

bool Packet::is_important() const
{
    return ((uint8_t)_tag) & 0b10000000;
}

Packet::Tag Packet::get_tag() const
{
    return _tag;
}

uint32_t Packet::get_sequence_counter() const
{
    return _sequence_counter;
}

const sf::IpAddress &Packet::get_sender_addr() const
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
    uint8_t tag = (uint8_t)_tag;
    tmp << tag;
    tmp << _sequence_counter;
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
    uint8_t tag;
    tmp >> tag;
    _tag = (Packet::Tag)tag;
    tmp >> _sequence_counter;
    append((uint8_t *)data + _header_size, size - _header_size);
}
