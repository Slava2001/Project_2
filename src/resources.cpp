#include "resources.hpp"
#include "settings.hpp"

#include <exception>

struct Resources Resources = {

};

void Resources::load()
{
    if (!fonts.main.loadFromFile(Settings.text.fonts_path.main))
    {
        throw std::runtime_error("Failed to load main font");
    }
}
