#include "gui-panel.hpp"
#include "util.hpp"
#include "logger.hpp"

using namespace GUI;

Panel::Panel(nlohmann::json &cfg, const Resources::Manager &res_mngr): 
    Base(cfg, res_mngr)
{
    _body_leave_color = color_from_string(cfg.value("body_color", "#000000"));
    _body_enter_color = _body_leave_color;
    if (cfg["body_enter_color"].is_string()) 
    {
        _body_enter_color = color_from_string(cfg.value("body_enter_color", "#000000"));
    }
    _body.setFillColor(_body_leave_color);
    sf::Vector2f panel_size;
    panel_size.x = cfg.value("width", 0);
    panel_size.y = cfg.value("height", 0);
    _body.setSize(sf::Vector2f(panel_size.x, panel_size.y));
}

void Panel::on_enter()
{
    _body.setFillColor(_body_enter_color);
}

void Panel::on_leave()
{
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
    target.draw(_body, states_copy);
    Base::draw(target, states);
}
