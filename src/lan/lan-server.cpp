#define LOG_LVL LOG_LVL_DEBUG
#include "logger.hpp"
#include "lan-server.hpp"
#include "debug-drawer.hpp"

using namespace Lan;

Server::Server(uint16_t port)
{
    _manager.start(port);
    _default_channel = _manager.get_default_channel();

}

Server::~Server()
{
    for (const auto &c: _clients) {
        _manager.close(c->_channel);
    }
    _manager.stop();
}

void Server::update()
{
    _manager.update();

    Packet recv;
    if (_default_channel->recv(recv) == Status::OK) {
        if (recv.get_tag() == Packet::TAG_REQUEST_SLOT) {
            log_info("Accept slot request from: ",
                     recv.get_sender_addr(), ":", recv.get_sender_port());

            std::shared_ptr<Client> client = std::make_shared<Client>();
            client->_channel = _manager.open(recv.get_sender_addr(), recv.get_sender_port());
            _clients.insert(client);
            Packet confirm(Packet::TAG_CONFIRM_CONNECT);
            client->_channel->send(confirm);
        } else {
            log_warn("Accept packet with unexpected tag:", recv.get_tag());
        }
    }

    Debug_drawer::add_string("client count: ", _clients.size());
    for (const auto &c: _clients) {
        client_update(c);
    }
    Debug_drawer::add_string("erase count: ", _clients_to_erase.size());

    if (_clients_to_erase.size() > 0) {
        for (const auto &c: _clients_to_erase) {
            _clients.erase(c);
        }
        _clients_to_erase.clear();
    }
}

void Server::client_disconnect(std::shared_ptr<Client> client)
{
    log_info("Disconnect client: ",
             client->_channel->get_addr().toString(), ":", client->_channel->get_port());
    _clients_to_erase.insert(client);
}

void Server::client_sends_to_others(std::shared_ptr<Client> client, const Packet &packet)
{
    for (const auto &c: _clients) {
        if (c != client) {
            if (c->_channel->send(packet) != Status::OK) {
                log_error("Failed to send packet to client");
                client_disconnect(client);
            }
        }
    }
}

void Server::client_update(std::shared_ptr<Client> client)
{
    Packet recv;
    Status status = client->_channel->recv(recv);
    switch (status) {
    case Status::OVERFLOW:
        log_info("Client channel overflow");
        client_disconnect(client);
    break;
    case Status::TIMEOUT:
        log_info("Client channel timeout");
        client_disconnect(client);
        break;
    case Status::ERROR:
        log_info("Client channel error");
        client_disconnect(client);
        break;
    case Status::OK:
        switch (recv.get_tag()) {
        case Packet::TAG_REQUEST_SLOT:
            log_info("Client again request slot");
            client_disconnect(client);
            break;
        case Packet::TAG_STRING:
            client_sends_to_others(client, recv);
            break;
        default:
            break;
        }
        break;
    default:
        break;
    }
}
