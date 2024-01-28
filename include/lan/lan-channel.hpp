#ifndef INCLUDE_LAN_LAN_CHANNEL
#define INCLUDE_LAN_LAN_CHANNEL

#include "SFML/Network.hpp"
#include <cstdint>
#include <queue>

namespace Lan {

constexpr unsigned packet_contend_max_len = 64;
constexpr unsigned packet_buffer_max_len = 64;

class Packet : public sf::Packet {
public:
    Packet();

    void set_important(bool flag);
    void set_tag(uint8_t tag);

    bool is_important() const;
    uint8_t get_tag() const;
    uint32_t get_sequence_counter() const;
    const sf::IpAddress get_sender_addr() const;
    uint16_t get_sender_port() const;

    const void* onSend(std::size_t& size) override;
    void onReceive(const void* data, std::size_t size) override;

private:
    static const unsigned _header_size = sizeof(uint8_t) + sizeof(uint32_t);

    sf::Packet tmp;
    bool _is_important;
    uint8_t _tag;
    uint32_t _sequence_counter;
    sf::IpAddress _addr;
    uint16_t _port;

    friend class Channel;
    void set_sequence_counter(uint32_t counter);
    void set_sender_addr(const sf::IpAddress &addr);
    void set_sender_port(uint16_t port);
};

enum class Status {
    OK,
    NOT_READY,
    TIMEOUT,
    ERROR
};

class Channel {
public:
    Channel();
    Status send(const struct Packet &packet);
    Status recv(struct Packet &packet);

private:
    sf::IpAddress _addr;
    uint16_t _port;

    std::queue<Packet> _send_buff;
    std::queue<Packet> _recv_buff;

    uint32_t _send_sequence_counter;
    uint32_t _recv_sequence_counter;

    friend class Manager;
    void set_addr(const sf::IpAddress &addr);
    void set_port(uint16_t port);
    bool has_packet_to_send();
    Packet *get_packet();
    void pop_packet();
    Status take_packet(Packet *packet, sf::IpAddress addr, uint16_t port);
};

}

#endif // INCLUDE_LAN_LAN_CHANNEL
