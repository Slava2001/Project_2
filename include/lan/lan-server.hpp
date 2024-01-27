#ifndef INCLUDE_LAN_SERVER_HPP
#define INCLUDE_LAN_SERVER_HPP

#include "SFML/Network.hpp"

namespace Lan {

class Server {
public:
    Server(uint16_t port);
    void update();
private:
    sf::UdpSocket _socket;
};

}

#endif // INCLUDE_LAN_SERVER_HPP
