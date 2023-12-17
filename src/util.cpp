#include "util.hpp"

#include "SFML/Graphics.hpp"

#include <exception>
#include <regex>

sf::Color color_from_string(std::string str)
{
    std::regex color_regex("^#([0-9a-fA-F]{2}){3,4}$");
    if (!regex_match(str, color_regex)) {
        throw std::runtime_error("Failed to parse color from string");
    }
    uint8_t r = (uint8_t)strtol(str.substr(1, 2).c_str(), NULL, 16);
    uint8_t g = (uint8_t)strtol(str.substr(3, 2).c_str(), NULL, 16);
    uint8_t b = (uint8_t)strtol(str.substr(5, 2).c_str(), NULL, 16);
    uint8_t a = 255;
    if (str.length() > 7) {
        a = (uint8_t)strtol(str.substr(7, 2).c_str(), NULL, 16);
    }
    return sf::Color(r, g, b, a);
}