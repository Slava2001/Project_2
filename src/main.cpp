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
    panel_1.setPosition(sf::Vector2f(100, 100));

    GUI::Panel panel_11;
    panel_1.add(&panel_11);

    GUI::Textbox tb1(sf::Vector2f(90, 16));
    panel_11.add(&tb1);
    GUI::Textbox tb2(sf::Vector2f(90, 16));
    panel_11.add(&tb2);

    tb1.setPosition(sf::Vector2f(5, 30));
    tb2.setPosition(sf::Vector2f(5, 60));

    panel_11.setPosition(sf::Vector2f(-100, 100));

    GUI::Panel panel_12;
    panel_1.add(&panel_12);
    GUI::Button bt1;
    panel_12.add(&bt1);
    GUI::Button bt2;
    panel_12.add(&bt2);
    GUI::Button bt3([](GUI::Button &b)
                    { b.set_text("Click again!"); });
    panel_12.add(&bt3);

    bt1.setPosition(sf::Vector2f(20, 20));
    bt2.setPosition(sf::Vector2f(20, 50));
    bt3.setPosition(sf::Vector2f(20, 80));

    panel_12.setPosition(sf::Vector2f(100, 100));

    while (window.isOpen())
    {
        sf::Event event;
        while (window.pollEvent(event))
        {
            if (event.type == sf::Event::Closed)
            {
                window.close();
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
        Debug_drawer::add_rect(sf::FloatRect(sf::Vector2f(mouse_pos.x, mouse_pos.y),
                                             sf::Vector2f(5, 5)));
        Debug_drawer::add_string("Mouse position: ", mouse_pos);

        gui.update(mouse_pos);

        window.clear(sf::Color(0x87cefa));
        window.draw(gui);
        window.draw(debug_drawer);
        window.display();
    }
    return 0;
}
