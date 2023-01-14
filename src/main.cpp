#include "settings.hpp"
#include "resources.hpp"
#include "debug-drawer.hpp"
#include "scene-manager.hpp"

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

    Scene::Manager mgr;
    mgr.load_scene(Scene::scene_ids::OPTIONS);
    while (window.isOpen())
    {
        sf::Event event;
        while (window.pollEvent(event))
        {
            if (event.type == sf::Event::Closed)
            {
                window.close();
            }
            mgr.event_handling(event);
        }

        frame_counter++;
        if (clock.getElapsedTime().asSeconds() > Settings::Debug::fps_update_periud)
        {
            current_fps = frame_counter / clock.restart().asSeconds();
            frame_counter = 0;
        }
        Debug_drawer::add_string("FPS: ", current_fps);

        mgr.update();

        window.clear(sf::Color(Settings::Screen::background_color));
        window.draw(mgr);
        window.draw(debug_drawer);
        window.display();
    }
    return 0;
}
