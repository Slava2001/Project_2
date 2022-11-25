#include "gui-base.hpp"
#include "SFML/Window.hpp"
#include "debug-drawer.hpp"

#include <algorithm>

using namespace GUI;

Base::Base() : _parent(nullptr), _bounds()
{
    _is_fixed = true;
    _id = "";
}

Base::Base(nlohmann::json &cfg, const Resources::Manager &res_mngr) : _parent(nullptr)
{
    _bounds.top = 0;
    _bounds.left = 0;
    _bounds.width = cfg.value("width", 0);
    _bounds.height = cfg.value("height", 0);
    _is_fixed = cfg.value("is_fixed", false);
    sf::Vector2f pos;
    pos.x = cfg.value("left", 0);
    pos.y = cfg.value("top", 0);
    setPosition(pos);
    _id = cfg.value("id", "");
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

std::string Base::get_id()
{
    return _id;
}

void Base::set_hitbox(sf::Vector2f hitbox)
{
    _bounds = sf::FloatRect(sf::Vector2f(0, 0), hitbox);
}

bool Base::is_fixed() const
{
    return _is_fixed;
}

void Base::on_click(const sf::Event::MouseButtonEvent &e)
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

void Base::on_press(const sf::Event::MouseButtonEvent &e)
{
}

void Base::on_release(const sf::Event::MouseButtonEvent &e)
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

void Base::on_mouse_move(const sf::Event::MouseMoveEvent &e)
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
