#include "debug-drawer.hpp"
#include "settings.hpp"
#include "resources.hpp"
#include "logger.hpp"
#include "util.hpp"

Debug_drawer *Debug_drawer::_instance = nullptr;

Debug_drawer::Debug_drawer(nlohmann::json &cfg)
{
    if (_instance) {
        log_fatal("Try create second debug draver instanse");
        throw std::runtime_error("Try create second debug draver instanse");
    }
    _instance = this;
    if (!cfg["resources"].is_null()) {
        _resources.load(cfg["resources"]);
    }
    _text_color = color_from_string(cfg.value("text_color", "#000000"));
}

template <>
void Debug_drawer::add_string(const std::string &str, sf::Vector2i val)
{
    std::stringstream _sstr;
    _sstr << str;
    _sstr << "(" << val.x << "; " << val.y << ")";
    _instance->_text_lines.push_back(_sstr.str());
    _sstr.clear();
}

template <>
void Debug_drawer::add_string(const std::string &str, sf::Vector2f val)
{
    std::stringstream _sstr;
    _sstr << str;
    _sstr << "(" << val.x << "; " << val.y << ")";
    _instance->_text_lines.push_back(_sstr.str());
    _sstr.clear();
}

void Debug_drawer::add_string(const std::string &str)
{
    _instance->_text_lines.push_back(str);
}

void Debug_drawer::add_rect(const sf::FloatRect &rec)
{
    _instance->_rects.push_back(rec);
}

void Debug_drawer::draw(sf::RenderTarget &target, const sf::RenderStates &states) const
{
    sf::RectangleShape rs;
    rs.setFillColor(sf::Color::Transparent);
    rs.setOutlineThickness(-1);
    rs.setOutlineColor(sf::Color::Yellow);
    for (sf::FloatRect &r : _instance->_rects)
    {
        rs.setPosition(sf::Vector2f(r.left, r.top));
        rs.setSize(sf::Vector2f(r.width, r.height));
        target.draw(rs);
    }
    _instance->_rects.clear();

    sf::Text tx(*_instance->_resources.get_font(DEFAULT_RESOURCE_NAME));
    tx.setCharacterSize(Settings.text.debug_text_size);
    tx.setFillColor(_text_color);
    tx.setOutlineThickness(2);
    for (std::size_t i = 0; i < _text_lines.size(); i++)
    {
        tx.setString(_text_lines[i]);
        tx.setPosition(sf::Vector2f(0, i * Settings.text.debug_text_size));
        target.draw(tx);
    }
    _instance->_text_lines.clear();
}
