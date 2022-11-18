#include "gui-base.hpp"
#include "SFML/Window.hpp"
#include "debug-drawer.hpp"

using namespace GUI;

Base::Base(sf::Vector2f hitbox, bool is_fixed) : _parent(nullptr),
                                                 _bounds(sf::Vector2f(0, 0), hitbox),
                                                 _is_fixed(is_fixed)
{
}

bool Base::update_hover(sf::Vector2i mouse_pos, Base *&hover)
{
    for (int i = _childes.size() - 1; i >= 0; i--)
    {
        if (_childes[i]->update_hover(mouse_pos - (sf::Vector2i)getPosition(), hover))
        {
            return true;
        }
    }

    if (getTransform().transformRect(_bounds).contains((sf::Vector2f)mouse_pos))
    {
        hover = this;
        return true;
    }
    return false;
}

bool Base::add(Base *ctrl)
{
    if (ctrl)
    {
        ctrl->_parent = this;
        _childes.push_back(ctrl);
        ctrl->setPosition(ctrl->getPosition() - (sf::Vector2f)get_global_position());
        return true;
    }
    return false;
}

void Base::erase(Base *ctrl)
{
    _childes.erase(std::find(_childes.begin(), _childes.end(), ctrl));
}

void Base::detach()
{
    _old_parent = _parent;
    _old_position = getPosition();

    if (_parent)
    {
        _parent->erase(this);
    }
    _parent = nullptr;
}

void Base::retach()
{
    _parent = _old_parent;
    if (_parent)
    {
        _parent->add(this);
    }
    setPosition(_old_position);
}

sf::Vector2i Base::get_global_position()
{
    if (_parent)
    {
        return (sf::Vector2i)getPosition() + _parent->get_global_position();
    }
    else
    {
        return (sf::Vector2i)getPosition();
    }
}

void Base::set_hitbox(sf::Vector2f hitbox)
{
    _bounds = sf::FloatRect(sf::Vector2f(0, 0), hitbox);
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

void Base::on_key_press(const sf::Event::KeyEvent &e)
{
}

void Base::on_drag(Base *&drag)
{
    drag = this;
    detach();
}

void Base::on_drop(Base *hover)
{
    if (!hover || !hover->add(this))
    {
        retach();
    }
}

void Base::on_input_text(const sf::Event::TextEvent &e)
{
}

void Base::draw(sf::RenderTarget &target, const sf::RenderStates &states) const
{
    sf::RenderStates states_copy(states.transform * getTransform());
    for (Base *c : _childes)
    {
        target.draw(*c, states_copy);
    }
}
