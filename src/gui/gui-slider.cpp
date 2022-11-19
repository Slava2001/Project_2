#include "gui-slider.hpp"
#include "debug-drawer.hpp"

#include <cmath>

using namespace GUI;

constexpr sf::Color Slider::_body_color = sf::Color(50, 50, 50);
constexpr sf::Color Slider::_arrow_color = sf::Color(250, 250, 250);

Slider::Slider(sf::Vector2f size, float min, float max, float step) : _size(size),
                                                                      _min(min),
                                                                      _max(max),
                                                                      _step(step)
{
    set_hitbox(_size);
    _body.setSize(_size);
    _body.setFillColor(_body_color);
    _arrow.setSize(sf::Vector2f(_size.y, _size.y));
    _arrow.setFillColor(sf::Color::Transparent);
    _arrow.setOutlineThickness(-3);
    _arrow.setOutlineColor(_arrow_color);
    _is_buttun_press = false;
    _step_in_pixel = (_size.x - _size.y) / ((_max - _min) / _step);
    _step_in_pixel = _step_in_pixel >= 1 ? _step_in_pixel : 1;
}

float Slider::get_value() const
{
    float value = _min + ((_max - _min) * _arrow.getPosition().x / (_size.x - _size.y));
    return round(value / _step) * _step;
}

void Slider::set_change_value_callback(std::function<void(Slider &s)> callback)
{
    _change_value_callback = callback;
}

void Slider::on_press()
{
    _is_buttun_press = true;
}

void Slider::on_release()
{
    _is_buttun_press = false;
}

void Slider::on_mouse_move(const sf::Event::MouseMoveEvent &e)
{
    if (_is_buttun_press)
    {
        sf::Vector2i mouse_pos(e.x, e.y);
        sf::Vector2f arrow_pos(mouse_pos - get_global_position());
        arrow_pos.x = std::max(0.f, std::min(arrow_pos.x - _size.y / 2, _size.x - _size.y));
        arrow_pos.x = round(arrow_pos.x / _step_in_pixel) * _step_in_pixel;
        arrow_pos.y = 0;
        _arrow.setPosition((sf::Vector2f)arrow_pos);
        if (_change_value_callback)
        {
            _change_value_callback(*this);
        }
    }
}

void Slider::draw(sf::RenderTarget &target, const sf::RenderStates &states) const
{
    sf::RenderStates states_copy(states.transform * getTransform());
    target.draw(_body, states_copy);
    target.draw(_arrow, states_copy);
}
