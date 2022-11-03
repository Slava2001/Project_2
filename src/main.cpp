#include "settings.hpp"
#include "resources.hpp"
#include "debug-drawer.hpp"

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
    while (window.isOpen()) {
        sf::Event event;
        while (window.pollEvent(event)) {
            if (event.type == sf::Event::Closed) {
                window.close();
            }
        }

        frame_counter++;
        if (clock.getElapsedTime().asSeconds() > Settings::Debug::fps_update_periud) {
            current_fps = frame_counter/clock.restart().asSeconds();
            frame_counter = 0;
        }

        Debug_drawer::add_string("FPS: ", current_fps);
        sf::Vector2i mouse_pos = sf::Mouse::getPosition(window);
        Debug_drawer::add_rect(sf::FloatRect(sf::Vector2f(mouse_pos.x, mouse_pos.y), 
                                             sf::Vector2f(5,5)));
        Debug_drawer::add_string("Mouse position:");
        Debug_drawer::add_string("Mouse X: ", mouse_pos.x);
        Debug_drawer::add_string("Mouse Y: ", mouse_pos.y);

        window.clear();
        window.draw(debug_drawer);
        window.display();
    }

    return 0;
}