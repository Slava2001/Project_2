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
    _input_channel(nullptr),
    _output_channel(nullptr)
{
    _text_out = _gui.get_elem<GUI::Textbox>("text_out");
    _gui.get_elem<GUI::Textbox>("input")->set_enter_callback([&](GUI::Textbox &t) {
        if (_output_channel) {
            Lan::Packet packet(Lan::Packet::TAG_STRING);
            packet << t.get_text();
            _output_channel->send(packet);
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
        _input_channel = _server.open();
        messager->set_visible(true);
        menu->set_visible(false);
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
            _output_channel = _server.open(addr.value(), port);
            messager->set_visible(true);
            menu->set_visible(false);
            Lan::Packet packet(Lan::Packet::TAG_STRING);
            _output_channel->send(packet);
        } else {
            ip_textbox->clear();
        }
    });
}

void Test_server_scene::update(float delta_time)
{
    _server.update();

    Lan::Packet packet;
    if (_input_channel && _input_channel->recv(packet) == Lan::Status::OK) {
        _output_channel = _server.open(packet.get_sender_addr(),
                                    packet.get_sender_port());
        *_text_out << "[" << packet.get_sender_addr().toString() << "] *connected*\n";
        Lan::Packet ans(Lan::Packet::TAG_STRING);
        ans << "Wellcom!";
        _output_channel->send(ans);
    }
    if (_output_channel && _output_channel->recv(packet) == Lan::Status::OK) {
        std::string str;
        packet >> str;
        *_text_out << "[" << packet.get_sender_addr().toString() << "] " << str << '\n';
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
