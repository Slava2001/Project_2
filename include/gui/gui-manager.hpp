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
        /// @brief Constructor
        Manager();
        /// @brief Update
        /// @param mouse_pos mouse position
        void update(sf::Vector2i mouse_pos);
        /// @brief Add GUI element
        /// @param focus poiter to GUI element
        void add(Base *focus);
        /// @brief Handling event
        /// @param e event
        void event_handling(const sf::Event &e);

        void draw(sf::RenderTarget &target, const sf::RenderStates &states) const;

    private:
        Base _controls;
        Base *_hover;
        Base *_drag;
        sf::Vector2i _drag_offset;
        Base *_pressed;
        bool _mouse_left_button_pressed;
        Base *_focus;

        /// @brief Update hover
        /// @param mouse_pos mouse position
        void update_hover(sf::Vector2i mouse_pos);
        /// @brief Update drag
        /// @param mouse_pos mouse position
        void update_drag(sf::Vector2i mouse_pos);
    };
}
#endif // INCLUDE_GUI_GUI_MANAGER_HPP
