#include "gui-panel.hpp"
#include "util.hpp"

using namespace GUI;

Panel::Panel(nlohmann::json &cfg) : Base(cfg)
{
    _body_leave_color = color_from_string(cfg.value("body_color", "#000000"));
    _head_leave_color = color_from_string(cfg.value("head_color", "#000000"));
    _body_enter_color = _body_leave_color;
    _head_enter_color = _head_leave_color;
    if (cfg["body_enter_color"].is_string()) 
    {
        _body_enter_color = color_from_string(cfg.value("body_enter_color", "#000000"));
    }
    if (cfg["head_enter_color"].is_string()) 
    {
        _head_enter_color = color_from_string(cfg.value("head_enter_color", "#000000"));
    }
    _head.setFillColor(_head_leave_color);
    _body.setFillColor(_body_leave_color);
    sf::Vector2f panel_size;
    panel_size.x = cfg.value("width", 0);
    panel_size.y = cfg.value("height", 0);
    int head_size = cfg.value("head_size", 0);
    _head.setSize(sf::Vector2f(panel_size.x, head_size));
    _body.setPosition(sf::Vector2f(0, head_size));
    _body.setSize(sf::Vector2f(panel_size.x, panel_size.y - head_size));
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
