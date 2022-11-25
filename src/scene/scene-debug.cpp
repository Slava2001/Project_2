#include "scene-manager.hpp"
#include "scene-debug.hpp"
#include "debug-drawer.hpp"
#include "logger.hpp"

using namespace Scene;

Debug_scene::Debug_scene(nlohmann::json &cfg):
    Base(cfg),
    _gui(cfg["ui_cfg"])
{
    _gui.get_elem<GUI::Button>("press_me_button")->set_click_callback([=](GUI::Button &) {
        log_info("Press...");
    });
}

void Debug_scene::update(float delta_time)
{
}

void Debug_scene::event_handling(const sf::Event &e)
{
    _gui.event_handling(e);
}

void Debug_scene::draw(sf::RenderTarget &target, const sf::RenderStates &states) const
{
    target.draw(_gui);
}
