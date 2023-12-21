#ifndef INCLUDE_RESOUECES_HPP
#define INCLUDE_RESOUECES_HPP

#include "SFML/Graphics.hpp"

struct Resources {
    struct Fonts {
        sf::Font main;
    } fonts;
    /// @brief Load resources.
    void load();
};

extern struct Resources Resources;

#endif // INCLUDE_RESOUECES_HPP
