#include "scene-manager.hpp"
#include "scene-test-server.hpp"
#include "debug-drawer.hpp"
#include "logger.hpp"

#include <sstream>

using namespace Scene;

Test_server_scene::Test_server_scene(nlohmann::json &cfg):
    Base(cfg),
    _gui(cfg["ui_cfg"]),
    _server(),
    _recv_channel(nullptr),
    _client_channel(nullptr),
    _is_server(false)
{
    _text_out = _gui.get_elem<GUI::Textbox>("text_out");
    _gui.get_elem<GUI::Textbox>("input")->set_enter_callback([&](GUI::Textbox &t) {
        Lan::Packet packet(Lan::Packet::TAG_STRING);
        packet << t.get_text();
        if (_is_server) {
            for (const auto &c: _client_channels) {
                c->send(packet);
            }
        } else {
            _client_channel->send(packet);
        }
        *_text_out << "[you]: " << t.get_text() << '\n';
        t.clear();
    });

    _gui.get_elem<GUI::Button>("server_btn")->set_click_callback([&](GUI::Button) {
        GUI::Panel *messager = _gui.get_elem<GUI::Panel>("messager");
        GUI::Panel *menu = _gui.get_elem<GUI::Panel>("menu");
        GUI::Textbox *port_textbox = _gui.get_elem<GUI::Textbox>("my_port_input");
        std::stringstream port_str(port_textbox->get_text());
        uint16_t port;
        port_str >> port;
        _server.start(port);
        _recv_channel = _server.get_default_channel();
        messager->set_visible(true);
        menu->set_visible(false);
        _is_server = true;
    });

    _gui.get_elem<GUI::Button>("client_btn")->set_click_callback([&](GUI::Button) {
        GUI::Panel *messager = _gui.get_elem<GUI::Panel>("messager");
        GUI::Panel *menu = _gui.get_elem<GUI::Panel>("menu");
        GUI::Textbox *port_textbox = _gui.get_elem<GUI::Textbox>("port_input");
        GUI::Textbox *ip_textbox = _gui.get_elem<GUI::Textbox>("ip_input");
        std::stringstream port_str(port_textbox->get_text());
        uint16_t port;
        port_str >> port;
        _server.start();

        std::optional<sf::IpAddress> addr = sf::IpAddress::resolve(ip_textbox->get_text());
        if (addr.has_value()) {
            _client_channel = _server.open(addr.value(), port);
            messager->set_visible(true);
            menu->set_visible(false);
            Lan::Packet packet(Lan::Packet::TAG_STRING);
            _client_channel->send(packet);
        } else {
            ip_textbox->clear();
        }
    });
}

void Test_server_scene::update(float delta_time)
{
    _server.update();

    if (_is_server) {
        Lan::Packet packet;
        if (_recv_channel && _recv_channel->recv(packet) == Lan::Status::OK) {
            Lan::Channel *client = _server.open(packet.get_sender_addr(),
                                                packet.get_sender_port());
            *_text_out << "[" << packet.get_sender_addr().toString() << ":"
                       << packet.get_sender_port() << "] *connected*\n";
            Lan::Packet ans(Lan::Packet::TAG_STRING);
            ans << "Wellcom!";
            client->send(ans);
            _client_channels.push_back(client);
        }

        for (std::size_t i = 0; i < _client_channels.size(); i++) {
            if (_client_channels[i]->get_status() != Lan::Status::OK) {
                *_text_out << "[" << _client_channels[i]->get_addr().toString() << ":"
                           << _client_channels[i]->get_port() << "] *disconnected*\n";
                _server.close(_client_channels[i]);
                _client_channels.erase(_client_channels.begin() + i);
            } else {
                Lan::Packet packet;
                if (_client_channels[i]->recv(packet) == Lan::Status::OK) {
                    std::string str;
                    packet >> str;
                    *_text_out << "[" << packet.get_sender_addr().toString() << ":"
                               << packet.get_sender_port() << "] " << str << '\n';
                    for (const auto &c: _client_channels) {
                        if (c != _client_channels[i]) {
                            c->send(packet);
                        }
                    }
                }
            }
        }
    } else {
        if (_client_channel) {
            if (_client_channel->get_status() != Lan::Status::OK) {
                _server.close(_client_channel);
                _client_channel = nullptr;
                GUI::Panel *messager = _gui.get_elem<GUI::Panel>("messager");
                GUI::Panel *menu = _gui.get_elem<GUI::Panel>("menu");
                messager->set_visible(false);
                menu->set_visible(true);
            } else {
                Lan::Packet packet;
                if (_client_channel->recv(packet) == Lan::Status::OK) {
                    std::string str;
                    packet >> str;
                    *_text_out << "[" << packet.get_sender_addr().toString() << ":"
                               << packet.get_sender_port() << "] " << str << '\n';
                }
            }
        }
    }
}

void Test_server_scene::event_handling(const sf::Event &e)
{
    _gui.event_handling(e);
}

void Test_server_scene::draw(sf::RenderTarget &target, const sf::RenderStates &states) const
{
    target.draw(_gui);
}
