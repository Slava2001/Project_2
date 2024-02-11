#ifndef INCLUDE_LAN_LAN_CLIENT_HPP
#define INCLUDE_LAN_LAN_CLIENT_HPP

#include "lan-manager.hpp"
#include "lan-channel.hpp"
#include "lan-packet.hpp"
#include "lan-object.hpp"

#include "SFML/Network.hpp"

#include <array>
#include <unordered_set>

namespace Lan {

    constexpr unsigned TICK_PER_SECOND = 8;
    constexpr sf::Time TICK_TIME = sf::milliseconds(1000/TICK_PER_SECOND);

    /// @brief Lan client that connects to the lan server and manages the lan object
    class Client {
    public:
        /// @brief Constructor
        /// @param addr server addres
        /// @param port server port
        Client(sf::IpAddress addr, uint16_t port);
        /// @brief Destructor
        ~Client();
        /// @brief Update client
        Status update();
        /// @brief Send packet
        /// @param packet packet to send
        /// @return channel status
        Status send(const Packet &packet);
        /// @brief Subscribe object on tag. After subscription, for each received
        ///        packet with the specified tag, the object recv method will be called
        /// @param object object that will be subscribed to tag
        /// @param tag tag to subscribe to
        void subscribe(Object *object, Packet::Tag tag);
        /// @brief Unsubscribe object on all tags and timer
        /// @param object object that will be unsubscribed
        void unsubscribe(Object *object);
        /// @brief Subscribe object on timer. After subscription, for each timer tick,
        ///        the object on_tick method will be called
        /// @param object object that will be subscribed to timer
        void subscribe_on_timer(Object *object);

    private:
        sf::Clock _timer;
        Manager _manager;
        Channel *_channel;
        std::array<std::unordered_set<Object *>, Packet::TAG_COUNT> _objects;
        std::unordered_set<Object *> _timered_objects;
        bool _is_connected;
        sf::IpAddress _server_addr;
        uint16_t _server_port;

    };

};

#endif // INCLUDE_LAN_LAN_CLIENT_HPP
