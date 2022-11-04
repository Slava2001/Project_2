#include "resources.hpp"

#include <exception>

namespace Resources
{
    namespace Fonts
    {
        sf::Font arial;
    }
}

#define RESOURCES_PATH "./resources"

void Resources::load()
{
    if (!Resources::Fonts::arial.loadFromFile(RESOURCES_PATH "/arial.ttf"))
    {
        throw std::runtime_error("Failed to load font");
    }
}
