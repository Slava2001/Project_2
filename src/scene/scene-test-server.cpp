#include "scene-manager.hpp"
#include "scene-test-server.hpp"
#include "debug-drawer.hpp"
#include "logger.hpp"

#include <sstream>

using namespace Scene;

Test_server_scene::Chat_obj::Chat_obj(GUI::Textbox *input, GUI::Textbox *output, Lan::Client &client):
    Object(client),
    _output(output)
{
    subscribe(Lan::Packet::TAG_STRING);
    input->set_enter_callback([&](GUI::Textbox &t) {
        std::string msg(t.get_text());
        t.clear();
        if (msg == "time") {
            subscribe_on_timer();
            return;
        }
        *_output << "[you]: " << msg << "\n";
        Lan::Packet pac(Lan::Packet::TAG_STRING);
        pac << msg;
        if (send(pac) != Lan::Status::OK) {
            log_error("Failed to send msg");
        }
    });
}

void Test_server_scene::Chat_obj::recv(Lan::Packet packet)
{
    std::string msg;
    packet >> msg;
    *_output << "[other]: " << msg << "\n";
}

void Test_server_scene::Chat_obj::on_tick()
{
    Lan::Packet pac(Lan::Packet::TAG_STRING);
    pac << std::string("timer msg");
    if (send(pac) != Lan::Status::OK) {
        log_error("Failed to send msg");
    }
}


Test_server_scene::Test_server_scene(nlohmann::json &cfg):
    Base(cfg),
    _gui(cfg["ui_cfg"])
{
    _text_out = _gui.get_elem<GUI::Textbox>("text_out");

    _gui.get_elem<GUI::Button>("server_btn")->set_click_callback([&](GUI::Button &b) {
        b.set_visible(false);
        GUI::Textbox *port_textbox = _gui.get_elem<GUI::Textbox>("my_port_input");
        std::stringstream port_str(port_textbox->get_text());
        uint16_t port;
        port_str >> port;
        _server = std::make_shared<Lan::Server>(port);
    });

    _gui.get_elem<GUI::Button>("client_btn")->set_click_callback([&](GUI::Button) {
        GUI::Panel *messager = _gui.get_elem<GUI::Panel>("messager");
        GUI::Panel *menu = _gui.get_elem<GUI::Panel>("menu");
        GUI::Textbox *port_textbox = _gui.get_elem<GUI::Textbox>("port_input");
        GUI::Textbox *ip_textbox = _gui.get_elem<GUI::Textbox>("ip_input");
        std::stringstream port_str(port_textbox->get_text());
        uint16_t port;
        port_str >> port;
        std::optional<sf::IpAddress> addr = sf::IpAddress::resolve(ip_textbox->get_text());
        if (addr.has_value()) {
            _client = std::make_shared<Lan::Client>(addr.value(), port);

            _obj = std::make_unique<Chat_obj>(_gui.get_elem<GUI::Textbox>("input"), _text_out,
                                              *_client.get());

            messager->set_visible(true);
            menu->set_visible(false);

        } else {
            ip_textbox->clear();
        }
    });
}

Test_server_scene::~Test_server_scene()
{
    _obj.reset();
    _client.reset();
}

void Test_server_scene::update(float delta_time)
{
    if (_server) {
        _server->update();
    }
    if (_client) {
        if (_client->update() == Lan::Status::TIMEOUT) {
            *_text_out << "Server timeout. Trying reconnect\n";
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
