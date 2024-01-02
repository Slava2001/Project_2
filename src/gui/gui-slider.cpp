#include "gui-slider.hpp"
#include "debug-drawer.hpp"
#include "util.hpp"

#include <cmath>

using namespace GUI;

Slider::Slider(nlohmann::json &cfg, const Resources::Manager &res_mngr): 
    Base(cfg, res_mngr)
{
    _body_color = color_from_string(cfg.value("body_color", "#000000"));
    _arrow_color = color_from_string(cfg.value("arrow_color", "#000000"));
    _min = cfg.value("min", 0.f);
    _max = cfg.value("max", 1.f);
    _step = cfg.value("step", 0.f);
    _size.x = cfg.value("width", 0);
    _size.y = cfg.value("height", 0);
    _body.setSize(_size);
    _body.setFillColor(_body_color);
    _arrow.setSize(sf::Vector2f(_size.y, _size.y));
    _arrow.setFillColor(sf::Color::Transparent);
    _arrow.setOutlineThickness(-3);
    _arrow.setOutlineColor(_arrow_color);
    _is_buttun_press = false;
    _step_in_pixel = _step > 0?((_size.x - _arrow.getSize().x) / ((_max - _min) / _step)): 1;
    _step_in_pixel = _step_in_pixel >= 1 ? _step_in_pixel : 1;
}

float Slider::get_value() const
{
    float value = _min + ((_max - _min) * _arrow.getPosition().x / (_size.x - _size.y));
    return round(value / _step) * _step;
}

void Slider::set_value(float val) 
{
    if (val < _min) {
        val = _min;
    }
    if (val > _max) {
        val = _max;
    }
    val = std::round(val/_step)*_step;
    sf::Vector2f pos(_arrow.getPosition());
    pos.x = ((val - _min) / (_max - _min)) * (_size.x - _size.y);
    _arrow.setPosition(pos);
    if (_change_value_callback)
    {
        _change_value_callback(*this);
    }
}

void Slider::set_change_value_callback(std::function<void(Slider &s)> callback)
{
    _change_value_callback = callback;
}

void Slider::on_press(const sf::Event::MouseButtonEvent &e)
{
    _is_buttun_press = true;
    update_arrow_pos(sf::Vector2i(e.x, e.y));
}

void Slider::on_release(const sf::Event::MouseButtonEvent &e)
{
    _is_buttun_press = false;
}

void Slider::on_mouse_move(const sf::Event::MouseMoveEvent &e)
{
    if (_is_buttun_press)
    {
        update_arrow_pos(sf::Vector2i(e.x, e.y));
    }
}

void Slider::update_arrow_pos(sf::Vector2i mouse_global_pos)
{
    sf::Vector2f arrow_pos(mouse_global_pos - get_global_position());
    arrow_pos.x = std::max(0.f, std::min(arrow_pos.x - _size.y / 2, _size.x - _size.y));
    arrow_pos.x = round(arrow_pos.x / _step_in_pixel) * _step_in_pixel;
    arrow_pos.y = 0;
    _arrow.setPosition((sf::Vector2f)arrow_pos);
    if (_change_value_callback)
    {
        _change_value_callback(*this);
    }
}

void Slider::draw(sf::RenderTarget &target, const sf::RenderStates &states) const
{
    sf::RenderStates states_copy(states.transform * getTransform());
    target.draw(_body, states_copy);
    target.draw(_arrow, states_copy);
}
