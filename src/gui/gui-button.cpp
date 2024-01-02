#include "gui-button.hpp"
#include "resources.hpp"
#include "settings.hpp"
#include "util.hpp"

using namespace GUI;

Button::Button(nlohmann::json &cfg, const Resources::Manager &res_mngr): 
    Base(cfg, res_mngr), 
    _on_click_callback(default_on_click_callback)
{
    sf::Vector2f size;
    size.x = cfg.value("width", 0);
    size.y = cfg.value("height", 0);
    _body.setSize(size);
    _body_color = color_from_string(cfg.value("body_color", "#000000"));
    _body.setFillColor(_body_color);
    sf::Color text_color = color_from_string(cfg.value("text_color", "#000000"));
    _text.setFillColor(text_color);
    _text.setCharacterSize(cfg.value("font_size", 0));
    _text.setFont(*res_mngr.get_font(cfg.value("font", DEFAULT_RESOURCE_NAME)));
    set_text(cfg.value("text", ""));
}

bool Button::add(Base *ctrl)
{
    return false;
}

void Button::on_press(const sf::Event::MouseButtonEvent &e)
{
    _body.setFillColor(sf::Color::Green);
}

void Button::on_release(const sf::Event::MouseButtonEvent &e)
{
    _body.setFillColor(_body_color);
}

void Button::on_enter()
{
    _body.setOutlineThickness(3);
}

void Button::on_leave()
{
    _body.setOutlineThickness(0);
}

void Button::on_click(const sf::Event::MouseButtonEvent &e)
{
    _on_click_callback(*this);
}

void Button::set_text(std::string str)
{
    _text.setString(str);
    _text.setPosition(sf::Vector2f((_body.getSize().x - _text.getLocalBounds().getSize().x)/2, 0));
}

void Button::draw(sf::RenderTarget &target, const sf::RenderStates &states) const
{
    sf::RenderStates states_copy(states.transform * getTransform());
    target.draw(_body, states_copy);
    target.draw(_text, states_copy);
    Base::draw(target, states);
}

void Button::set_click_callback(std::function<void(Button &)> callback)
{
    _on_click_callback = callback;
}

void Button::default_on_click_callback(Button &btn)
{
}
