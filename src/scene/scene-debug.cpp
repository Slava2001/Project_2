#include "scene-debug.hpp"
#include "debug-drawer.hpp"

using namespace Scene;

Debug_scene::Debug_scene(Manager &mgr) : Base(mgr),
                                         _gui(),
                                         _panel_1(),
                                         _tb_output(96, 16, 5),
                                         _panel_2(),
                                         _tb(90),
                                         _btn([&](GUI::Button &b)
                                              { load_scene(scene_ids::DEBUG); }),
                                         _slider(sf::Vector2f(100, 20), 0, 255, 255 / 4.f)
{
    _gui.add(&_panel_1);
    _panel_1.setPosition(sf::Vector2f(210, 200));

    _panel_1.add(&_tb_output);
    _tb_output.setPosition(sf::Vector2f(2, 15));
    _tb_output.set_changeable(false);
    _tb_output.set_scroling(true);

    _gui.add(&_panel_2);
    _panel_2.setPosition(sf::Vector2f(100, 200));

    _panel_2.add(&_tb);
    _tb.setPosition(sf::Vector2f(5, 30));
    _tb.set_enter_callback([&](GUI::Textbox &t)
                           { std::stringstream input(t.get_text());
                            _tb_output << "\n" << ">" << input.str();
                            std::string cmd;
                            input >> cmd;
                            if (cmd == "add") {
                                int a,b;
                                input >> a >> b;
                                if (input.fail()) {
                                    _tb_output << "\n Error! \n add [a] [b]";
                                } else {
                                    _tb_output << "\n Sum: " << (a + b);
                                }
                            } else {
                                _tb_output << "\nUnknown \ncommand: \n" << cmd;
                            }
                            t.clear(); });

    _panel_2.add(&_btn);
    _btn.setPosition(sf::Vector2f(20, 80));

    _gui.add(&_slider);
    _slider.setPosition(sf::Vector2f(50, 50));
}

void Debug_scene::update()
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
