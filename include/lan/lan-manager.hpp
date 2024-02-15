#ifndef INCLUDE_LAN_LAN_MANAGER
#define INCLUDE_LAN_LAN_MANAGER

#include "lan-channel.hpp"

#include "SFML/Network.hpp"
#include <cstdint>
#include <map>

namespace Lan {

class Manager {
public:
    /// @brief Constructor
    Manager();
    /// @brief Start manager
    /// @param port listening port
    void start(uint16_t port = sf::UdpSocket::AnyPort);
    /// @brief Stop manager
    void stop();
    /// @brief Get default channel.
    ///        This channel receives packets that did not go to other channels
    /// @return pointer to default channel
    Recv_channel* get_default_channel();
    /// @brief Open a channel and bind it to the specified address
    /// @param addr address for binding
    /// @param port port for binding
    /// @return pointer to a channel bound to the specified address
    Channel* open(sf::IpAddress addr, uint16_t port);
    /// @brief Close opened channel
    /// @param channel Channel to close
    void close(Channel *channel);
    /// @brief Get listening port
    /// @return listening port
    uint16_t get_port();
    /// @brief Update manager. Distribute received messages across channels and
    ///        send outgoing messages
    void update();

private:
    std::map<uint64_t, Channel> _channels;
    sf::UdpSocket _socket;
    bool _is_started;
    Recv_channel _default_channel;

    /// @brief Get addres hash.
    /// @param addr addres
    /// @param port port
    /// @return addres hash
    uint64_t get_addr_hash(const sf::IpAddress &addr, uint16_t port);

};

}

#endif // INCLUDE_LAN_LAN_MANAGER
