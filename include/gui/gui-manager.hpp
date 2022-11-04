#ifndef INCLUDE_GUI_GUI_MANAGER_HPP
#define INCLUDE_GUI_GUI_MANAGER_HPP

#include "gui-base.hpp"

#include "gui-panel.hpp"

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
        std::vector<Base *> _controls;
        Base *_focus;
        Base *_hover;
        Base *_drag;
        Base *_presed;
        sf::Vector2i _drag_offset;
        bool _is_mouse_left_buttun_presed;

        void update_hover(sf::Vector2i mouse_pos);
    };
}
#endif // INCLUDE_GUI_GUI_MANAGER_HPP
