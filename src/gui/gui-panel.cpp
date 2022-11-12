#include "gui-panel.hpp"

using namespace GUI;

constexpr sf::Color Panel::_body_enter_color(100, 100, 100, 200);
constexpr sf::Color Panel::_head_enter_color(100, 100, 100, 250);
constexpr sf::Color Panel::_body_leave_color(0, 0, 0, 200);
constexpr sf::Color Panel::_head_leave_color(0, 0, 0, 250);
constexpr sf::Vector2f Panel::_panel_size(100, 100);
constexpr int Panel::_head_size = 15;

Panel::Panel() : Base(_panel_size, false)
{
    _head.setFillColor(_head_leave_color);
    _body.setFillColor(_body_leave_color);

    _head.setSize(sf::Vector2f(_panel_size.x, _head_size));
    _body.setPosition(sf::Vector2f(0, _head_size));
    _body.setSize(sf::Vector2f(_panel_size.x, _panel_size.y - _head_size));
}

void Panel::on_enter()
{
    _head.setFillColor(_head_enter_color);
    _body.setFillColor(_body_enter_color);
}

void Panel::on_leave()
{
    _head.setFillColor(_head_leave_color);
    _body.setFillColor(_body_leave_color);
}

void Panel::on_drag(Base *&drag)
{
    drag = this;
    detach();
}

void Panel::on_drop(Base *hover)
{
    _parent = _old_parent;
    if (_parent)
    {
        _parent->add(this);
    }
}

void Panel::draw(sf::RenderTarget &target, const sf::RenderStates &states) const
{
    sf::RenderStates states_copy(states.transform * getTransform());
    target.draw(_head, states_copy);
    target.draw(_body, states_copy);
    Base::draw(target, states);
}
