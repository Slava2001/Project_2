#ifndef INCLUDE_GUI_GUI_MANAGER_HPP
#define INCLUDE_GUI_GUI_MANAGER_HPP

#include "gui-base.hpp"

#include "gui-panel.hpp"
#include "gui-textbox.hpp"
#include "gui-button.hpp"

#include "SFML/System.hpp"

namespace GUI
{
    class Manager : public sf::Drawable
    {
    public:
        Manager();
        void update(sf::Vector2i mouse_pos);
        void add(Base *focus);
        void draw(sf::RenderTarget &target, const sf::RenderStates &states) const;

    private:
        Base _controls;
        Base *_hover;
        Base *_drag;
        sf::Vector2i _drag_offset;
        Base *_pressed;
        bool _mouse_left_button_pressed;

        void update_hover(sf::Vector2i mouse_pos);
        void update_drag(sf::Vector2i mouse_pos);
    };
}
#endif // INCLUDE_GUI_GUI_MANAGER_HPP
