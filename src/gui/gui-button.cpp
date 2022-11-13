#include "gui-button.hpp"
#include "resources.hpp"
#include "settings.hpp"

using namespace GUI;

constexpr sf::Vector2f Button::_size(65, 15);

Button::Button(std::function<void(Button &)> callback) : Base(_size, true),
                                                         _body(_size),
                                                         _on_click_callback(callback)
{
    _body.setFillColor(sf::Color::White);
    _text.setFillColor(sf::Color::Black);
    _text.setFont(Resources::Fonts::main);
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
    _on_click_callback(*this);
}

void Button::set_text(std::string str)
{
    _text.setString(str);
    _text.setPosition(sf::Vector2f((_body.getSize().x - _text.getLocalBounds().getSize().x) / 2, 0));
}

void Button::draw(sf::RenderTarget &target, const sf::RenderStates &states) const
{
    sf::RenderStates states_copy(states.transform * getTransform());
    target.draw(_body, states_copy);
    target.draw(_text, states_copy);
    Base::draw(target, states);
}

void Button::default_on_click_callback(Button &btn)
{
}
