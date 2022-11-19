#include "gui-manager.hpp"
#include "debug-drawer.hpp"

using namespace GUI;

Manager::Manager() : _controls()
{
    _hover = &_controls;
    _drag = nullptr;
    _pressed = nullptr;
    _focus = nullptr;
    _mouse_left_button_pressed = false;
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
        if (_hover != &_controls)
        {
            _hover->on_leave();
        }
        _hover = &_controls;
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
        else if (!_mouse_left_button_pressed)
        {
            if (_hover != &_controls)
            {
                _pressed = _hover;
                _pressed->on_press();
                if (!_hover->is_fixed())
                {
                    _drag_offset = _hover->get_global_position() - mouse_pos;
                    _hover->on_drag(_drag);
                    _drag->setPosition((sf::Vector2f)(mouse_pos + _drag_offset));
                }
            }
            if (_pressed != _focus)
            {
                if (_focus)
                {
                    _focus->on_defocus();
                }
                _focus = _pressed;
                if (_focus)
                {
                    _focus->on_focus();
                }
            }
            _mouse_left_button_pressed = true;
        }
    }
    else
    {
        if (_drag)
        {
            _drag->on_drop(_hover);
            _drag = nullptr;
        }
        if (_pressed)
        {
            if (_pressed == _hover)
            {
                _pressed->on_click();
            }
            _pressed->on_release();
            _pressed = nullptr;
        }
        _mouse_left_button_pressed = false;
    }
}

void Manager::event_handling(const sf::Event &e)
{
    switch (e.type)
    {
    case sf::Event::MouseButtonPressed:
    case sf::Event::MouseButtonReleased:
    {
        sf::Vector2i mouse_pos(e.mouseButton.x, e.mouseButton.y);
        update_hover(mouse_pos);
        update_drag(mouse_pos);
    }
    break;
    case sf::Event::MouseMoved:
    {
        sf::Vector2i mouse_pos(e.mouseMove.x, e.mouseMove.y);
        update_hover(mouse_pos);
        update_drag(mouse_pos);
    }
    break;
    default:
        break;
    }

    if (_focus)
    {
        switch (e.type)
        {
        case sf::Event::KeyPressed:
            _focus->on_key_press(e.key);
            break;
        case sf::Event::TextEntered:
            _focus->on_input_text(e.text);
            break;
        case sf::Event::MouseMoved:
            _focus->on_mouse_move(e.mouseMove);
            break;
        default:
            break;
        }
    }
}

void Manager::draw(sf::RenderTarget &target, const sf::RenderStates &states) const
{
    target.draw(_controls, states);
    if (_drag)
    {
        target.draw(*_drag, states);
    }
}
