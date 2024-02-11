#ifndef INCLUDE_LAN_LAN_CHANNEL
#define INCLUDE_LAN_LAN_CHANNEL

#include "lan-packet.hpp"
#include "lan-recv-channel.hpp"

#include "SFML/Network.hpp"
#include "SFML/System.hpp"

#include <cstdint>
#include <queue>

namespace Lan {

    // time before re-sending an important packet if confirmation is not received
    constexpr sf::Time TIME_BETWEEN_RESENDS = sf::milliseconds(125);
    // The number of attempts to send an important message.
    // If within the specified number of times it was not possible to receive a response,
    // the connection is considered broken
    constexpr unsigned RESENDS_COUNT = 24;
    // Maximum downtime. If the channel does not receive packets within this time,
    // it sends a PING to check communication
    constexpr sf::Time TIME_BETWEEN_PINGS = sf::milliseconds(1000);

    /// @brief Lan channel
    class Channel: public Recv_channel {
    public:
        /// @brief Constructor
        Channel();
        /// @brief Send packet
        /// @param packet packet to send
        /// @return channel status
        Status send(const Packet &packet);
        /// @brief reset channel state
        virtual void reset() override;
        /// @brief Get the address to which the channel is attached
        /// @return address
        const sf::IpAddress& get_addr() const override;
        /// @brief Get the port to which the channel is attached
        /// @return port
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
        unsigned _resends_count;
        sf::Clock _time_since_last_receipt;
        /// @brief Send an important package
        /// @param packet packet send
        /// @return channel status
        Status send_important(const Packet &packet);
        /// @brief Send an not important package
        /// @param packet packet send
        /// @return channel status
        Status send_not_important(const Packet &packet) override;

        friend class Manager;
        /// @brief Set the address to which the channel is attached
        /// @param addr addres to set
        void set_addr(const sf::IpAddress &addr);
        /// @brief Set the port to which the channel is attached
        /// @param port port to set
        void set_port(uint16_t port);
        /// @brief Check if has packet to send
        /// @return true if the send queue is not empty, false otherwise
        bool has_packet_to_send() override;
        /// @brief Get packet to send
        /// @return packet to send
        Packet *get_packet() override;
        /// @brief Remove firs packet fron queue
        void pop_packet() override;
        /// @brief Take received packet
        /// @param packet received packet
        /// @param addr sender addres
        /// @param port sender port
        /// @return channel status
        Status take_packet(Packet *packet, sf::IpAddress addr, uint16_t port) override;
    };
}

#endif // INCLUDE_LAN_LAN_CHANNEL
