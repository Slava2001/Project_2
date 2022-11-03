#ifndef INCLUDE_DEBUG_DRAWER_HPP
#define INCLUDE_DEBUG_DRAWER_HPP

#include "SFML/Graphics.hpp"

#include <vector>
#include <string>
#include <sstream>

class Debug_drawer: public sf::Drawable {
public:

    static void add_string(const std::string &str);
    template <typename T>
    static void add_string(const std::string &str, T val);
    static void add_rect(const sf::FloatRect &rec);
    
    void draw(sf::RenderTarget& target, const sf::RenderStates& states) const override; 
private:

    static std::vector<std::string> _text_lines;
    static std::vector<sf::FloatRect> _rects;
};

template <typename T>
void Debug_drawer::add_string(const std::string &str, T val) 
{
    std::stringstream _sstr;
    _sstr << str;
    _sstr << val;
    _text_lines.push_back(_sstr.str());
    _sstr.clear();
}

#endif // INCLUDE_DEBUG_DRAWER_HPP