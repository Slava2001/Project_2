#ifndef INCLUDE_DEBUG_DRAWER_HPP
#define INCLUDE_DEBUG_DRAWER_HPP

#include "SFML/Graphics.hpp"
#include "resources.hpp"
#include "nlohmann/json.hpp"

#include <vector>
#include <string>
#include <sstream>

/// @brief Static class for show debug info
class Debug_drawer : public sf::Drawable
{
public:
    /// Init debug drawer by config
    /// @param cfg json config
    Debug_drawer(nlohmann::json &cfg);
    /// @brief Add string to debug output
    /// @param str string to add
    static void add_string(const std::string &str);
    /// @brief Add string whis some value to debug output
    /// @tparam T value type
    /// @param str value prefix
    /// @param val value to show
    template <typename T>
    static void add_string(const std::string &str, T val);
    /// @brief Add transparent rectangle to debug output
    /// @param rec rectangle to add
    static void add_rect(const sf::FloatRect &rec);

    void draw(sf::RenderTarget &target, const sf::RenderStates &states) const override;

private:
    std::vector<std::string> _text_lines;
    std::vector<sf::FloatRect> _rects;
    Resources::Manager _resources;
    static Debug_drawer *_instance;
    sf::Color _text_color;
};

template <>
void Debug_drawer::add_string(const std::string &str, sf::Vector2i val);
template <>
void Debug_drawer::add_string(const std::string &str, sf::Vector2f val);
template <typename T>
void Debug_drawer::add_string(const std::string &str, T val)
{
    std::stringstream _sstr;
    _sstr << str;
    _sstr << val;
    _instance->_text_lines.push_back(_sstr.str());
    _sstr.clear();
}
#endif // INCLUDE_DEBUG_DRAWER_HPP
