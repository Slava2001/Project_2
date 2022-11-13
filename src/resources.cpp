#include "resources.hpp"

#include <exception>

namespace Resources
{
    namespace Fonts
    {
        sf::Font main;
    }
}

#define RESOURCES_PATH "./resources"

void Resources::load()
{
    if (!Resources::Fonts::main.loadFromFile(RESOURCES_PATH "/UbuntuMono-R.ttf"))
    {
        throw std::runtime_error("Failed to load font");
    }
}
