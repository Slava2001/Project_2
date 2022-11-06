#include "gui-button.hpp"
#include "resources.hpp"
#include "settings.hpp"

using namespace GUI;

Button::Button() : Base(sf::Vector2f(65, 15), true),
                   _body(sf::Vector2f(65, 15))
{
    _body.setFillColor(sf::Color::White);
    _text.setFillColor(sf::Color::Black);
    _text.setFont(Resources::Fonts::arial);
    _text.setCharacterSize(Settings::Text::debug_text_size);
    set_text("Click me!");
}

bool Button::add(Base *ctrl)
{
    return false;
}

void Button::on_press()
{
    _body.setFillColor(sf::Color::Green);
}

void Button::on_release()
{
    _body.setFillColor(sf::Color::White);
}

void Button::on_enter()
{
    _body.setOutlineThickness(3);
}

void Button::on_leave()
{
    _body.setOutlineThickness(0);
}

void Button::on_click()
{
    set_text("Click again!");
}

void Button::set_text(std::string str)
{
    _text.setString(str);
    _text.setPosition(sf::Vector2f((_body.getSize().x - _text.getLocalBounds().getSize().x) / 2, 0));
}

void Button::draw(sf::RenderTarget &target, const sf::RenderStates &states) const
{
    target.draw(_body, states.transform * getTransform());
    target.draw(_text, states.transform * getTransform());
    Base::draw(target, states);
}
