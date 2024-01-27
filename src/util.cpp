#include "util.hpp"
#include "logger.hpp"

#include "SFML/Graphics.hpp"

#include <fstream>
#include <exception>
#include <regex>

sf::Color color_from_string(std::string str)
{
    std::regex color_regex("^#([0-9a-fA-F]{2}){3,4}$");
    if (!regex_match(str, color_regex)) {
        log_fatal("Failed to parse \"", str, "\" as color");
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

nlohmann::json cfg_from_file(const std::string &path)
{
    std::ifstream cfg_file(path);
    if (!cfg_file.is_open()) {
        log_fatal("Failed to open file: ", path);
        throw std::runtime_error("Failed to open file");
    }
    nlohmann::json cfg = nlohmann::json::parse(cfg_file);
    cfg_file.close();
    return cfg;
}

