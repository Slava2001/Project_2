#ifndef INCLUDE_LAN_LAN_PACKET_HPP
#define INCLUDE_LAN_LAN_PACKET_HPP

#include "SFML/Network.hpp"

#include <cstdint>

namespace Lan {

    /// @brief packet to be transmitted over the channel
    /// Format:
    /// tag, 1 byte, package tag, also defines the importance of the package:
    ///     tag 0-127 not important packages,
    ///     tag 128-255 important packages
    /// sequence counter, 4 byte, necessary for packet control.
    ///     Separate counters are used for important and unimportant packets
    /// payload dependensid tag
    class Packet : public sf::Packet {
    public:

        enum Tag {
            // not important tags:
            TAG_UNDEFINED = 0x00,
            CHANNEL_TAG_ACK = 0x01,
            TAG_STRING,
            // important tags:
            CHANNEL_TAG_PING = 0x80,
            TAG_REQUEST_SLOT = 0x81,
            TAG_CONFIRM_CONNECT = 0x82,
            TAG_COUNT = 0x100
        };

        /// @brief Constructor
        Packet();
        /// @brief Constructor
        /// @param tag packet tag
        Packet(Tag tag);
        /// @brief Check if the package is important
        /// @return true if the packet is important (tag > 127), false otherwise
        bool is_important() const;
        /// @brief Get packet tag
        /// @return packet tag
        Tag get_tag() const;
        /// @brief Get packet sequence counter
        /// @return packet sequence counter
        uint32_t get_sequence_counter() const;
        /// @brief Get packet sender addres
        /// @return packet sender addres
        const sf::IpAddress &get_sender_addr() const;
        /// @brief Get packet sender port
        /// @return packet sender addres
        uint16_t get_sender_port() const;

        const void* onSend(std::size_t& size) override;
        void onReceive(const void* data, std::size_t size) override;

    private:
        static const unsigned _header_size = sizeof(uint8_t) + sizeof(uint32_t);

        sf::Packet _tmp;
        Tag _tag;
        uint32_t _sequence_counter;
        sf::IpAddress _addr;
        uint16_t _port;

        friend class Channel;
        friend class Recv_channel;
        /// @brief Set sequence counter
        /// @param counter counter to set
        void set_sequence_counter(uint32_t counter);
        /// @brief Set sender addres
        /// @param addr addr to set
        void set_sender_addr(const sf::IpAddress &addr);
        /// @brief Set sender addres
        /// @param addr addr to set
        void set_sender_port(uint16_t port);
    };
}

#endif // INCLUDE_LAN_LAN_PACKET_HPP
