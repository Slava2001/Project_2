#ifndef INCLUDE_LAN_LAN_CHANNEL
#define INCLUDE_LAN_LAN_CHANNEL

#include "lan-packet.hpp"
#include "lan-recv-channel.hpp"

#include "SFML/Network.hpp"
#include "SFML/System.hpp"

#include <cstdint>
#include <queue>

namespace Lan {

constexpr sf::Time time_between_resends = sf::milliseconds(100);
constexpr unsigned resends_count = 3;
constexpr sf::Time time_between_pings = sf::milliseconds(1000);

class Channel: public Recv_channel {
public:
    Channel();
    Status send(const struct Packet &packet);
    virtual void reset() override;
    const sf::IpAddress& get_addr() const override;
    uint16_t get_port() const override;
private:
    sf::IpAddress _addr;
    uint16_t _port;

    std::queue<Packet> _send_important_buff;

    uint32_t _send_sequence_counter;
    uint32_t _send_important_sequence_counter;
    uint32_t _recv_sequence_counter;
    uint32_t _recv_important_sequence_counter;

    bool _waiting_ack;
    sf::Clock _waiting_time;
    unsigned _resend_count;
    sf::Clock _time_since_last_receipt;

    Status send_important(const struct Packet &packet);
    Status send_not_important(const struct Packet &packet) override;

    friend class Manager;
    void set_addr(const sf::IpAddress &addr);
    void set_port(uint16_t port);
    bool has_packet_to_send() override;
    Packet *get_packet() override;
    void pop_packet() override;
    Status take_packet(Packet *packet, sf::IpAddress addr, uint16_t port) override;
};

}

#endif // INCLUDE_LAN_LAN_CHANNEL
