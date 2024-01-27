#ifndef INCLUDE_UTIL_HPP
#define INCLUDE_UTIL_HPP

#include "SFML/Graphics.hpp"
#include "nlohmann/json.hpp"

/// @brief Converting input string to SFML color
/// @param str input string
/// @return color
sf::Color color_from_string(std::string str);
/// @brief Loading json config from file
/// @param path path to file
/// @return loaded config
nlohmann::json cfg_from_file(const std::string &path);

#endif // INCLUDE_UTIL_HPP
