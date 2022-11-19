#ifndef INCLUDE_GUI_GUI_MANAGER_HPP
#define INCLUDE_GUI_GUI_MANAGER_HPP

#include "gui-base.hpp"

#include "gui-panel.hpp"
#include "gui-textbox.hpp"
#include "gui-button.hpp"
#include "gui-slider.hpp"

#include "SFML/System.hpp"

namespace GUI
{
    class Manager : public sf::Drawable
    {
    public:
        /// @brief Constructor
        Manager();
        /// @brief Add GUI element
        /// @param ctrl poiter to GUI element
        void add(Base *ctrl);
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
        /// @param e mouse move event
        void update_hover(const sf::Event::MouseMoveEvent &e);
        /// @brief Try drag hovered element
        /// @param e mouse button event
        void drag(const sf::Event::MouseButtonEvent &e);
        /// @brief Drop drageged element
        /// @param e mouse button event
        void drop(const sf::Event::MouseButtonEvent &e);
        /// @brief Update dragged element
        /// @param e mouse move event
        void update_dragged(const sf::Event::MouseMoveEvent &e);
    };
}
#endif // INCLUDE_GUI_GUI_MANAGER_HPP
