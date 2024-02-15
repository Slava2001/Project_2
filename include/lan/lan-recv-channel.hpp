#ifndef INCLUDE_LAN_LAN_RECV_CHANNEL
#define INCLUDE_LAN_LAN_RECV_CHANNEL

#include "lan-packet.hpp"

#include "SFML/Network.hpp"
#include "SFML/System.hpp"

#include <cstdint>
#include <queue>

namespace Lan {

    // packet queue size
    constexpr unsigned PACKET_BUFFER_MAX_LEN = 64;

    enum class Status {
        OK,
        NOT_READY,
        TIMEOUT,
        ERROR,
        OVERFLOW
    };

    /// @brief Receiving channel. This channel can only be used to receive packets.
    class Recv_channel {
    public:
        /// @brief Constructor
        Recv_channel();
        /// @brief Get received packet
        /// @param packet received packet
        /// @return channel status
        Status recv(Packet &packet);
        /// @brief Get channel status
        /// @return channel status
        Status get_status();
        /// @brief reset channel state
        virtual void reset();

    protected:
        Status _status;
        std::queue<Packet> _recv_buff;
        std::queue<Packet> _send_buff;

        /// @brief Send confirmation of receipt of the package
        /// @param packet pointer to the packet to be acknowledged
        virtual void send_ack(const Packet *packet);
        /// @brief Send an important package
        /// @param packet packet send
        /// @return channel status
        virtual Status send_not_important(const Packet &packet);

        friend class Manager;
        /// @brief Get the address to which the channel is attached
        /// @return address
        virtual const sf::IpAddress& get_addr() const;
        /// @brief Get the port to which the channel is attached
        /// @return port
        virtual uint16_t get_port() const;
        /// @brief Check if has packet to send
        /// @return true if the send queue is not empty, false otherwise
        virtual bool has_packet_to_send();
        /// @brief Get packet to send
        /// @return packet to send
        virtual Packet *get_packet();
        /// @brief Remove firs packet fron send queue
        virtual void pop_packet();
        /// @brief Take received packet
        /// @param packet received packet
        /// @param addr sender addres
        /// @param port sender port
        /// @return channel status
        virtual Status take_packet(Packet *packet, sf::IpAddress addr, uint16_t port);
    };
}

#endif // INCLUDE_LAN_LAN_RECV_CHANNEL
