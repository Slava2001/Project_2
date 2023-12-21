#ifndef INCLUDE_GUI_GUI_PANEL_HPP
#define INCLUDE_GUI_GUI_PANEL_HPP

#include "gui-base.hpp"
#include "SFML/System.hpp"

#include "nlohmann/json.hpp"

namespace GUI
{
    class Panel : public Base
    {
    public:
        /// @brief Constructor
        /// @param cfg panel config
        Panel(nlohmann::json &cfg);
        /// @brief cursor enter callback
        void on_enter() override;
        /// @brief cursor leave callback
        void on_leave() override;
        /// @brief on drag callback
        /// @param[out] drag the pointer to the dragged element
        void on_drag(Base *&drag) override;
        /// @brief on drop callback
        /// @param[in] hover current hovered element
        void on_drop(Base *hover) override;

        void draw(sf::RenderTarget &target, const sf::RenderStates &states) const;

    private:
        sf::Color _body_enter_color;
        sf::Color _head_enter_color;
        sf::Color _body_leave_color;
        sf::Color _head_leave_color;
        
        sf::RectangleShape _head;
        sf::RectangleShape _body;
    };
};
#endif // INCLUDE_GUI_GUI_PANEL_HPP
