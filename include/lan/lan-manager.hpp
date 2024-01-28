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

    void start(uint16_t port);
    void stop();
    Channel* open();
    Channel* open(sf::IpAddress addr, uint16_t port);
    void close(Channel *channel);

    void update();

private:
    std::map<uint32_t, Channel> _channels;
    sf::UdpSocket _socket;
    bool _is_started;
};

}

#endif // INCLUDE_LAN_LAN_MANAGER
