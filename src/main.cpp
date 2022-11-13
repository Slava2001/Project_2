#include "settings.hpp"
#include "resources.hpp"
#include "debug-drawer.hpp"
#include "gui-manager.hpp"

#include "SFML/Graphics.hpp"
#include "SFML/System.hpp"

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
    panel_1.setPosition(sf::Vector2f(210, 100));

    GUI::Textbox tb_output(96, 16, 5);
    panel_1.add(&tb_output);
    tb_output.setPosition(sf::Vector2f(2, 15));
    tb_output.set_changeable(false);
    tb_output.set_scroling(true);

    GUI::Panel panel_2;
    gui.add(&panel_2);
    panel_2.setPosition(sf::Vector2f(100, 100));
    GUI::Textbox tb(90);
    panel_2.add(&tb);
    tb.setPosition(sf::Vector2f(5, 30));
    GUI::Button btn([&](GUI::Button &b)
                    { tb_output << tb.get_text() << "\n";
                      tb.clear(); });
    panel_2.add(&btn);
    btn.setPosition(sf::Vector2f(20, 60));

    while (window.isOpen())
    {
        sf::Event event;
        while (window.pollEvent(event))
        {
            if (event.type == sf::Event::Closed)
            {
                window.close();
            }
            if (event.type == sf::Event::KeyPressed)
            {
                gui.on_key_presed(event.key);
            }
        }

        frame_counter++;
        if (clock.getElapsedTime().asSeconds() > Settings::Debug::fps_update_periud)
        {
            current_fps = frame_counter / clock.restart().asSeconds();
            frame_counter = 0;
        }

        Debug_drawer::add_string("FPS: ", current_fps);
        sf::Vector2i mouse_pos = sf::Mouse::getPosition(window);

        gui.update(mouse_pos);

        window.clear(sf::Color(0x87cefa));
        window.draw(gui);
        window.draw(debug_drawer);
        window.display();
    }
    return 0;
}
