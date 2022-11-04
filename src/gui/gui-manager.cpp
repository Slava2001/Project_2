#include "gui-manager.hpp"
#include "debug-drawer.hpp"

using namespace GUI;

Manager::Manager()
{
    _focus = nullptr;
    _hover = nullptr;
    _drag = nullptr;
    _presed = nullptr;
}

void Manager::add(Base *ctrl)
{
    if (ctrl)
    {
        _controls.push_back(ctrl);
    }
}

void Manager::update_hover(sf::Vector2i mouse_pos)
{
    for (Base *c : _controls)
    {
        if (c->getGlobalBounds().contains((sf::Vector2f)mouse_pos))
        {
            if (_hover)
            {
                _hover->on_leave();
                _hover = nullptr;
            }
            _hover = c;
            c->on_enter();
            break;
        }
        else if (_hover == c)
        {
            _hover->on_leave();
            _hover = nullptr;
        }
    }
}

void Manager::update(sf::Vector2i mouse_pos)
{
    update_hover(mouse_pos);

    if (sf::Mouse::isButtonPressed(sf::Mouse::Button::Left))
    {
        if (!_is_mouse_left_buttun_presed)
        {
            _presed = _hover;
            if (_presed)
            {
                _presed->on_press();
            }

            if (_focus != _presed)
            {
                if (_focus)
                {
                    _focus->on_defocus();
                }
                if (_presed)
                {
                    _presed->on_focus();
                }
            }
            _focus = _presed;
        }
        _is_mouse_left_buttun_presed = true;
    }
    else
    {
        if (_is_mouse_left_buttun_presed)
        {
            if (_presed)
            {
                _presed->on_release();
                if (_hover == _presed)
                {
                    _presed->on_click();
                }
            }
            if (_drag)
            {
                _drag->on_release();
            }
        }

        _presed = nullptr;
        _drag = nullptr;
        _is_mouse_left_buttun_presed = false;
    }

    if (_presed && !_presed->is_fixed())
    {
        _drag = _presed;
        _drag_offset = (sf::Vector2i)_drag->getPosition() - mouse_pos;
        _presed = nullptr;
    }

    if (_drag)
    {
        _drag->setPosition((sf::Vector2f)(mouse_pos + _drag_offset));
    }

    for (Base *c : _controls)
    {
        c->update(mouse_pos - (sf::Vector2i)c->getPosition());
    }
}

void Manager::draw(sf::RenderTarget &target, const sf::RenderStates &states) const
{
    for (Base *c : _controls)
    {
        if (_drag != c)
        {
            target.draw(*c, states);
        }
    }
    if (_drag)
    {
        target.draw(*_drag, states);
    }
}
