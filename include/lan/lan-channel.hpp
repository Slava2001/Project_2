#ifndef INCLUDE_LAN_LAN_CHANNEL
#define INCLUDE_LAN_LAN_CHANNEL

#include "lan-packet.hpp"

#include "SFML/Network.hpp"

#include <cstdint>
#include <queue>

namespace Lan {

constexpr unsigned packet_contend_max_len = 64;
constexpr unsigned packet_buffer_max_len = 64;

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
    std::queue<Packet> _send_important_buff;
    std::queue<Packet> _recv_buff;

    uint32_t _send_sequence_counter;
    uint32_t _send_important_sequence_counter;
    uint32_t _recv_sequence_counter;
    uint32_t _recv_important_sequence_counter;

    bool _waiting_ack;

    Status send_important(const struct Packet &packet);
    Status send_not_important(const struct Packet &packet);

    friend class Manager;
    void set_addr(const sf::IpAddress &addr);
    void set_port(uint16_t port);
    const sf::IpAddress& get_addr();
    uint16_t get_port();
    bool has_packet_to_send();
    Packet *get_packet();
    void pop_packet();
    Status take_packet(Packet *packet, sf::IpAddress addr, uint16_t port);
};

}

#endif // INCLUDE_LAN_LAN_CHANNEL
