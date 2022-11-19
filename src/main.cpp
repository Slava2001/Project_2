#include "settings.hpp"
#include "resources.hpp"
#include "debug-drawer.hpp"
#include "gui-manager.hpp"

#include "SFML/Graphics.hpp"
#include "SFML/System.hpp"

#include <sstream>

int main()
{
    Resources::load();

    sf::RenderWindow window(sf::VideoMode(sf::Vector2u(Settings::Window::width,
                                                       Settings::Window::height)),
                            Settings::Window::title);
    window.setFramerateLimit(Settings::Window::fps_limit);

    Debug_drawer debug_drawer;

    sf::Clock clock;
    int frame_counter = 0;
    int current_fps = 0;

    GUI::Manager gui;

    GUI::Panel panel_1;
    gui.add(&panel_1);
    panel_1.setPosition(sf::Vector2f(210, 200));

    GUI::Textbox tb_output(96, 16, 5);
    panel_1.add(&tb_output);
    tb_output.setPosition(sf::Vector2f(2, 15));
    tb_output.set_changeable(false);
    tb_output.set_scroling(true);

    GUI::Panel panel_2;
    gui.add(&panel_2);
    panel_2.setPosition(sf::Vector2f(100, 200));
    GUI::Textbox tb(90);
    panel_2.add(&tb);
    tb.setPosition(sf::Vector2f(5, 30));
    tb.set_enter_callback([&](GUI::Textbox &t)
                          { std::stringstream input(t.get_text());
                            tb_output << "\n" << ">" << input.str();
                            std::string cmd;
                            input >> cmd;
                            if (cmd == "add") {
                                int a,b;
                                input >> a >> b;
                                if (input.fail()) {
                                    tb_output << "\n Error! \n add [a] [b]";
                                } else {
                                    tb_output << "\n Sum: " << (a + b);
                                }
                            } else {
                                tb_output << "\nUnknown \ncommand: \n" << cmd;
                            }
                            t.clear(); });
    GUI::Button btn([&](GUI::Button &b)
                    { static int c = 0;
                      tb_output << "\n" << c << ": [" << (char)c << "]";
                      c = (c + 1) % 256; });
    panel_2.add(&btn);
    btn.setPosition(sf::Vector2f(20, 80));

    GUI::Slider slider_r(sf::Vector2f(100, 20), 0, 255, 255 / 4.f);
    gui.add(&slider_r);
    slider_r.setPosition(sf::Vector2f(50, 50));
    uint8_t color_r = 0;
    slider_r.set_change_value_callback([&](GUI::Slider &s)
                                       { color_r = s.get_value(); });
    GUI::Slider slider_g(sf::Vector2f(100, 20), 0, 255, 255 / 4.f);
    gui.add(&slider_g);
    slider_g.setPosition(sf::Vector2f(50, 80));
    uint8_t color_g = 0;
    slider_g.set_change_value_callback([&](GUI::Slider &s)
                                       { color_g = s.get_value(); });
    GUI::Slider slider_b(sf::Vector2f(100, 20), 0, 255, 255 / 4.f);
    gui.add(&slider_b);
    slider_b.setPosition(sf::Vector2f(50, 110));
    uint8_t color_b = 0;
    slider_b.set_change_value_callback([&](GUI::Slider &s)
                                       { color_b = s.get_value(); });

    while (window.isOpen())
    {
        sf::Event event;
        while (window.pollEvent(event))
        {
            if (event.type == sf::Event::Closed)
            {
                window.close();
            }

            gui.event_handling(event);
        }

        frame_counter++;
        if (clock.getElapsedTime().asSeconds() > Settings::Debug::fps_update_periud)
        {
            current_fps = frame_counter / clock.restart().asSeconds();
            frame_counter = 0;
        }

        Debug_drawer::add_string("FPS: ", current_fps);

        window.clear(sf::Color(color_r, color_g, color_b));
        window.draw(gui);
        window.draw(debug_drawer);
        window.display();
    }
    return 0;
}
