#include "gui-manager.hpp"
#include "debug-drawer.hpp"

using namespace GUI;

Manager::Manager() : _controls()
{
    _hover = nullptr;
    _drag = nullptr;
}

void Manager::add(Base *ctrl)
{
    if (ctrl)
    {
        _controls.add(ctrl);
    }
}

void Manager::update_hover(sf::Vector2i mouse_pos)
{
    Base *hover = _hover;
    if (_controls.update_hover(mouse_pos, hover))
    {
        if (hover != _hover)
        {
            hover->on_enter();
            if (_hover)
            {
                _hover->on_leave();
            }
            _hover = hover;
        }
    }
    else
    {
        if (_hover)
        {
            _hover->on_leave();
        }
        _hover = nullptr;
    }
}

void Manager::update_drag(sf::Vector2i mouse_pos)
{
    if (sf::Mouse::isButtonPressed(sf::Mouse::Button::Left))
    {
        if (_drag)
        {
            _drag->setPosition((sf::Vector2f)(mouse_pos + _drag_offset));
        }
        else
        {
            if (_hover && !_hover->is_fixed())
            {
                _drag = _hover;
                _drag_offset = _drag->get_global_position() - mouse_pos;
                _hover->detach();
            }
        }
    }
    else
    {
        if (_drag)
        {
            if (_hover)
            {
                _hover->add(_drag);
            }
            else
            {
                _drag->retach();
            }
            _drag = nullptr;
        }
    }
}

void Manager::update(sf::Vector2i mouse_pos)
{
    _controls.update(mouse_pos);

    update_hover(mouse_pos);
    update_drag(mouse_pos);
    Debug_drawer::add_string("_hover:  ", _hover);
    Debug_drawer::add_string("_drag:   ", _drag);
}

void Manager::draw(sf::RenderTarget &target, const sf::RenderStates &states) const
{
    target.draw(_controls, states);
    if (_drag)
    {
        target.draw(*_drag, states);
    }
}
