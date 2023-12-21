#include "debug-drawer.hpp"
#include "settings.hpp"
#include "resources.hpp"

std::vector<std::string> Debug_drawer::_text_lines;
std::vector<sf::FloatRect> Debug_drawer::_rects;

template <>
void Debug_drawer::add_string(const std::string &str, sf::Vector2i val)
{
    std::stringstream _sstr;
    _sstr << str;
    _sstr << "(" << val.x << "; " << val.y << ")";
    _text_lines.push_back(_sstr.str());
    _sstr.clear();
}

template <>
void Debug_drawer::add_string(const std::string &str, sf::Vector2f val)
{
    std::stringstream _sstr;
    _sstr << str;
    _sstr << "(" << val.x << "; " << val.y << ")";
    _text_lines.push_back(_sstr.str());
    _sstr.clear();
}

void Debug_drawer::add_string(const std::string &str)
{
    _text_lines.push_back(str);
}

void Debug_drawer::add_rect(const sf::FloatRect &rec)
{
    _rects.push_back(rec);
}

void Debug_drawer::draw(sf::RenderTarget &target, const sf::RenderStates &states) const
{
    sf::RectangleShape rs;
    rs.setFillColor(sf::Color::Transparent);
    rs.setOutlineThickness(-1);
    rs.setOutlineColor(sf::Color::Yellow);
    for (sf::FloatRect &r : _rects)
    {
        rs.setPosition(sf::Vector2f(r.left, r.top));
        rs.setSize(sf::Vector2f(r.width, r.height));
        target.draw(rs);
    }
    _rects.clear();

    sf::Text tx;
    tx.setFont(Resources.fonts.main);
    tx.setCharacterSize(Settings.text.debug_text_size);

    for (std::size_t i = 0; i < _text_lines.size(); i++)
    {
        tx.setString(_text_lines[i]);
        tx.setPosition(sf::Vector2f(0, i * Settings.text.debug_text_size));
        target.draw(tx);
    }
    _text_lines.clear();
}
