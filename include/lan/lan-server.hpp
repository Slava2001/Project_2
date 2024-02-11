#ifndef INCLUDE_LAN_LAN_SERVER
#define INCLUDE_LAN_LAN_SERVER

#include "lan-manager.hpp"
#include "lan-channel.hpp"
#include "lan-object.hpp"
#include "lan-packet.hpp"

#include "SFML/Network.hpp"

#include <array>
#include <unordered_set>

namespace Lan {

    /// @brief Lan server that accepts clients and manages them
    class Server {
    public:
        /// @brief Constructor
        /// @param port
        Server(uint16_t port);
        /// @brief Destructor
        ~Server();
        /// @brief Update server
        void update();

    private:
        struct Client {
            Channel *_channel;
        };
        Manager _manager;
        Recv_channel *_default_channel;
        std::unordered_set<std::shared_ptr<Client>> _clients;
        std::unordered_set<std::shared_ptr<Client>> _clients_to_erase;

        void client_update(std::shared_ptr<Client> client);
        void client_disconnect(std::shared_ptr<Client> client);
        void client_sends_to_others(std::shared_ptr<Client> client, const Packet &recv);

    };

};

#endif // INCLUDE_LAN_LAN_SERVER
