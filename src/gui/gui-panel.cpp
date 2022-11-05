#include "gui-panel.hpp"

using namespace GUI;

Panel::Panel() : Base(sf::Vector2f(100, 100), false)
{
    _head.setSize(sf::Vector2f(100, 15));
    _head.setFillColor(sf::Color(0, 0, 0, 250));
    _body.setPosition(sf::Vector2f(0, 15));
    _body.setSize(sf::Vector2f(100, 85));
    _body.setFillColor(sf::Color(0, 0, 0, 200));
}

void Panel::on_enter()
{
    _head.setFillColor(sf::Color(100, 100, 100, 250));
    _body.setFillColor(sf::Color(100, 100, 100, 200));
}
void Panel::on_leave()
{
    _head.setFillColor(sf::Color(0, 0, 0, 250));
    _body.setFillColor(sf::Color(0, 0, 0, 200));
}

void Panel::draw(sf::RenderTarget &target, const sf::RenderStates &states) const
{
    sf::RenderStates states_copy(states.transform * getTransform());
    target.draw(_head, states_copy);
    target.draw(_body, states_copy);
    Base::draw(target, states);
}
