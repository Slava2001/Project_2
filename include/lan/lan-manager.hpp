#ifndef INCLUDE_LAN_LAN_MANAGER
#define INCLUDE_LAN_LAN_MANAGER

#include "lan-channel.hpp"

#include "SFML/Network.hpp"
#include <cstdint>
#include <map>

namespace Lan {

class Manager {
public:
    Manager();

    void start(uint16_t port = sf::UdpSocket::AnyPort);
    void stop();
    Recv_channel* get_default_channel();
    Channel* open(sf::IpAddress addr, uint16_t port);
    void close(Channel *channel);

    uint16_t get_port();
    void update();

private:
    std::map<uint64_t, Channel> _channels;
    sf::UdpSocket _socket;
    bool _is_started;
    Recv_channel _default_channel;

    uint64_t get_addr_hash(const sf::IpAddress &addr, uint16_t port);

};

}

#endif // INCLUDE_LAN_LAN_MANAGER
