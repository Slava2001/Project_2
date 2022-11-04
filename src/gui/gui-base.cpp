#include "gui-base.hpp"
#include "SFML/Window.hpp"
#include "debug-drawer.hpp"

using namespace GUI;

Base::Base(sf::Vector2f hitbox, bool is_fixed) : _bounds(sf::Vector2f(0, 0), hitbox),
                                                 _is_fixed(is_fixed)
{
    _is_mouse_left_button_click = false;
    _is_mouse_hover = false;
}

bool Base::contains(sf::Vector2f point) const
{
    return getTransform().transformRect(_bounds).contains(point);
}

void Base::update(sf::Vector2i mose_pos)
{
}

bool Base::is_fixed() const
{
    return _is_fixed;
}

void Base::on_click()
{
}

void Base::on_enter()
{
}

void Base::on_leave()
{
}

void Base::on_focus()
{
}

void Base::on_defocus()
{
}

void Base::on_press()
{
}

void Base::on_release()
{
}

void Base::draw(sf::RenderTarget &target, const sf::RenderStates &states) const
{
}
