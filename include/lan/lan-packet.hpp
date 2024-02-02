#ifndef INCLUDE_LAN_LAN_PACKET_HPP
#define INCLUDE_LAN_LAN_PACKET_HPP

#include "SFML/Network.hpp"

#include <cstdint>

namespace Lan {

class Packet : public sf::Packet {
public:

    enum Tag {
        // not important tags:
        TAG_UNDEFINED = 0x00,
        CHANNEL_TAG_ACK = 0x01,
        // important tags:
        CHANNEL_TAG_PING = 0x80,
        TAG_STRING,
        TAG_COUNT = 0x100
    };

    Packet();
    Packet(Tag tag);
    bool is_important() const;
    Tag get_tag() const;
    uint32_t get_sequence_counter() const;
    const sf::IpAddress &get_sender_addr() const;
    uint16_t get_sender_port() const;

    const void* onSend(std::size_t& size) override;
    void onReceive(const void* data, std::size_t size) override;

private:
    static const unsigned _header_size = sizeof(uint8_t) + sizeof(uint32_t);

    sf::Packet tmp;
    Tag _tag;
    uint32_t _sequence_counter;
    sf::IpAddress _addr;
    uint16_t _port;

    friend class Channel;
    friend class Recv_channel;
    void set_sequence_counter(uint32_t counter);
    void set_sender_addr(const sf::IpAddress &addr);
    void set_sender_port(uint16_t port);
};
}


#endif // INCLUDE_LAN_LAN_PACKET_HPP
