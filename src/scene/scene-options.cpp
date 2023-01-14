#include "scene-options.hpp"
#include "settings.hpp"
#include "debug-drawer.hpp"

using namespace Scene;

Options_scene::Options_scene(Manager &mgr) : Base(mgr),
                                             _background_color_slider_r(sf::Vector2f(100, 20), 0, 255),
                                             _background_color_slider_g(sf::Vector2f(100, 20), 0, 255),
                                             _background_color_slider_b(sf::Vector2f(100, 20), 0, 255),
                                             _load_debug_scene_button([&](GUI::Button &b)
                                                                      { load_scene(scene_ids::DEBUG); })
{

    _gui.add(&_load_debug_scene_button);

    _background_color_slider_r.setPosition(sf::Vector2f(Settings::Window::width / 2 - 50, 100));
    _background_color_slider_r.set_change_value_callback([](GUI::Slider &s)
                                                         { Settings::Screen::background_color = (Settings::Screen::background_color & 0x00ffffff) |
                                                                                                (uint32_t)s.get_value() << 24; });

    _background_color_slider_g.setPosition(sf::Vector2f(Settings::Window::width / 2 - 50, 150));
    _background_color_slider_g.set_change_value_callback([](GUI::Slider &s)
                                                         { Settings::Screen::background_color = (Settings::Screen::background_color & 0xff00ffff) |
                                                                                                (uint32_t)s.get_value() << 16; });

    _background_color_slider_b.setPosition(sf::Vector2f(Settings::Window::width / 2 - 50, 200));
    _background_color_slider_b.set_change_value_callback([](GUI::Slider &s)
                                                         { Settings::Screen::background_color = (Settings::Screen::background_color & 0xffff00ff) |
                                                                                                (uint32_t)s.get_value() << 8; });
    _gui.add(&_background_color_slider_r);
    _gui.add(&_background_color_slider_g);
    _gui.add(&_background_color_slider_b);
}

void Options_scene::update()
{
}

void Options_scene::event_handling(const sf::Event &e)
{
    _gui.event_handling(e);
}

void Options_scene::draw(sf::RenderTarget &target, const sf::RenderStates &states) const
{
    target.draw(_gui);
}
