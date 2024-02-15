#ifndef INCLUDE_LAN_LAN_OBJECT_HPP
#define INCLUDE_LAN_LAN_OBJECT_HPP

#include "lan-channel.hpp"
#include "lan-packet.hpp"

#include "SFML/Network.hpp"

namespace Lan {
    class Client;

    /// @brief Lan object. An object that can send and receive
    ///        messages by subscribing to a tag
    class Object {
    public:
        /// @brief Rescive packet callback
        /// @param packet packet to recive
        /// @return channel status
        virtual void recv(Packet packet) = 0;
        /// @brief Timer callback. used to send periodic messages,
        ///        such as the coordinates of an object or its status
        virtual void on_tick();
    protected:
        /// @brief Constructor
        /// @param client lan client
        Object(Client &client);
        /// @brief Destructor
        ~Object();
        /// @brief Send packet
        /// @param packet packet to send
        /// @return channel status
        Status send(const Packet &packet);
        /// @brief Subscribe on tag. After subscription, for each received
        ///        packet with the specified tag, the recv method will be called
        /// @param tag tag to subscribe to
        void subscribe(Packet::Tag tag);
        /// @brief Subscribe on timer. After subscription, for each timer tick,
        ///        the on_tick method will be called
        void subscribe_on_timer();

    private:
        Client &_client;

    };
};

#endif // INCLUDE_LAN_LAN_OBJECT_HPP
