#include "gui-manager.hpp"
#include "debug-drawer.hpp"
#include "logger.hpp"

#include "nlohmann/json.hpp"

using namespace GUI;

Manager::Manager() : _controls()
{
    _hover = &_controls;
    _drag = nullptr;
    _pressed = nullptr;
    _focus = nullptr;
    _mouse_left_button_pressed = false;
}

Manager::Manager(nlohmann::json &cfg) : _controls()
{
    _hover = &_controls;
    _drag = nullptr;
    _pressed = nullptr;
    _focus = nullptr;
    _mouse_left_button_pressed = false;

    if (!cfg["resources"].is_null()) {
        _resources.load(cfg["resources"]);
    }
    if (!cfg["layout"].is_null()) {
        create_gui_tree(&_controls, cfg["layout"]);
    }
}

void Manager::create_gui_tree(Base *ctl, nlohmann::json &cfg)
{
    if (cfg["childes"].is_null()) {
        return;
    }
    if (!cfg["childes"].is_array()) {
        throw std::runtime_error("Unexpected childes param type");
    }
    for (auto& childe_cfg: cfg["childes"]) {
        std::shared_ptr<Base> elem = create_gui_element(childe_cfg);
        _dynamic_elements.push_back(elem);
        elem->setPosition(elem->getPosition() + (sf::Vector2f)ctl->get_global_position());
        ctl->add(elem.get());
        create_gui_tree(elem.get(), childe_cfg);
    }
}

std::shared_ptr<Base> Manager::create_gui_element(nlohmann::json &cfg)
{
    if (cfg["type"].is_null()) {
        throw std::runtime_error("Element type does not specified");
    }
    if (!cfg["type"].is_string()) {
        throw std::runtime_error("Unexpected type param type");
    }

    std::shared_ptr<Base> ptr;
    std::string type = cfg["type"];
    if (type == "panel") {
        ptr = std::make_shared<Panel>(cfg, _resources);
    } else if (type == "textbox") {
        ptr = std::make_shared<Textbox>(cfg, _resources);
    } else if (type == "button") {
        ptr = std::make_shared<Button>(cfg, _resources);
    } else if (type == "slider") {
        ptr = std::make_shared<Slider>(cfg, _resources);
    } else {
        throw std::runtime_error("Unexpected gui element type");
    }
    return ptr;
}

void Manager::update_hover(const sf::Event::MouseMoveEvent &e)
{
    Base *hover = _hover;
    sf::Vector2i mouse_pos(e.x, e.y);
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

void Manager::drag(const sf::Event::MouseButtonEvent &e)
{
    if (e.button == sf::Mouse::Button::Left)
    {
        if (_hover != &_controls)
        {
            _pressed = _hover;
            _pressed->on_press(e);
            if (!_hover->is_fixed())
            {
                sf::Vector2i mouse_pos(e.x, e.y);
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
    }
}

void Manager::drop(const sf::Event::MouseButtonEvent &e)
{
    if (e.button == sf::Mouse::Button::Left)
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
                _pressed->on_click(e);
            }
            _pressed->on_release(e);
            _pressed = nullptr;
        }
    }
}

void Manager::update_dragged(const sf::Event::MouseMoveEvent &e)
{
    if (_drag)
    {
        sf::Vector2i mouse_pos(e.x, e.y);
        _drag->setPosition((sf::Vector2f)(mouse_pos + _drag_offset));
    }
}

void Manager::event_handling(const sf::Event &e)
{
    switch (e.type)
    {
    case sf::Event::MouseButtonPressed:
        drag(e.mouseButton);
        break;
    case sf::Event::MouseButtonReleased:
        drop(e.mouseButton);
        break;
    case sf::Event::MouseMoved:
        update_hover(e.mouseMove);
        update_dragged(e.mouseMove);
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
