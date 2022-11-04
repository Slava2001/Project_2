#include "settings.hpp"
#include "resources.hpp"

#include <SFML/Graphics.hpp>

int main() 
{
    Resources::load();

    sf::RenderWindow window(sf::VideoMode(sf::Vector2u(Settings::Window::width,
                                                       Settings::Window::height)), 
                            Settings::Window::title);
    window.setFramerateLimit(Settings::Window::fps_limit);
    
    sf::CircleShape shape(100.f);
    shape.setFillColor(sf::Color::Green);

    while (window.isOpen()) {
        sf::Event event;
        while (window.pollEvent(event)) {
            if (event.type == sf::Event::Closed) {
                window.close();
            }
        }

        window.clear();
        window.draw(shape);
        window.display();
    }

    return 0;
}