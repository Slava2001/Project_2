#define LOG_LVL LOG_LVL_INFO
#include "logger.hpp"
#include "lan-server.hpp"

#include "SFML/Network.hpp"

using namespace Lan;

Server::Server(uint16_t port)
{
    if (_socket.bind(port) != sf::Socket::Status::Done) {
        log_fatal("Failed to bind socket to port: ", port);
        throw std::runtime_error("Failed to bind socket");
    }
    _socket.setBlocking(false);
}

void Server::update()
{
    uint8_t buff[1024];
    std::size_t received = 0;
    std::optional<sf::IpAddress> client_addr;
    uint16_t client_port;
    sf::Socket::Status status;
    status = _socket.receive(buff, 1024, received, client_addr, client_port);
    if (status == sf::Socket::Status::Error) {
        log_fatal("Failed to receive data. Socket error");
        throw std::runtime_error("Failed to receive data");
    }
    if (status == sf::Socket::Status::Done) {
        log_info("Server recv: ", (const char *)buff);
    }
}
