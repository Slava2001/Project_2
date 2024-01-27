#include "scene-manager.hpp"
#include "scene-test-server.hpp"
#include "debug-drawer.hpp"
#include "logger.hpp"
#include "lan-server.hpp"

using namespace Scene;

Test_server_scene::Test_server_scene(nlohmann::json &cfg):
    Base(cfg),
    _gui(cfg["ui_cfg"]),
    _server(12345)
{

}

void Test_server_scene::update(float delta_time)
{
    _server.update();
}

void Test_server_scene::event_handling(const sf::Event &e)
{
    _gui.event_handling(e);
}

void Test_server_scene::draw(sf::RenderTarget &target, const sf::RenderStates &states) const
{
    target.draw(_gui);
}
