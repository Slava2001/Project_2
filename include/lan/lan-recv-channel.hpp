#ifndef INCLUDE_LAN_LAN_RECV_CHANNEL
#define INCLUDE_LAN_LAN_RECV_CHANNEL

#include "lan-packet.hpp"

#include "SFML/Network.hpp"
#include "SFML/System.hpp"

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

class Recv_channel {
public:
    Recv_channel();
    Status recv(struct Packet &packet);
    Status get_status();
    virtual void reset();

protected:
    Status _status;
    std::queue<Packet> _recv_buff;
    std::queue<Packet> _send_buff;

    virtual void send_ack(const Packet *packet);
    virtual Status send_not_important(const struct Packet &packet);

    friend class Manager;
    virtual const sf::IpAddress& get_addr() const;
    virtual uint16_t get_port() const;
    virtual bool has_packet_to_send();
    virtual Packet *get_packet();
    virtual void pop_packet();
    virtual Status take_packet(Packet *packet, sf::IpAddress addr, uint16_t port);
};

}

#endif // INCLUDE_LAN_LAN_RECV_CHANNEL
