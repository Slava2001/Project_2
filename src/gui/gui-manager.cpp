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
    for (int i = _controls.size() - 1; i >= 0; i--)
    {
        if (_controls[i]->contains((sf::Vector2f)mouse_pos))
        {
            if (_hover)
            {
                _hover->on_leave();
                _hover = nullptr;
            }
            _hover = _controls[i];
            _controls[i]->on_enter();
            break;
        }
        else if (_hover == _controls[i])
        {
            _hover->on_leave();
            _hover = nullptr;
        }
    }
}

void Manager::update(sf::Vector2i mouse_pos)
{
    if (_drag)
    {
        _drag->setPosition((sf::Vector2f)(mouse_pos + _drag_offset));
    }

    for (Base *c : _controls)
    {
        c->update(mouse_pos - (sf::Vector2i)c->getPosition());
    }

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
        _controls.erase(std::find(_controls.begin(), _controls.end(), _drag));
        _controls.push_back(_drag);
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
