#ifndef INCLUDE_GUI_GUI_BUTTON_HPP
#define INCLUDE_GUI_GUI_BUTTON_HPP

#include "gui-base.hpp"
#include "SFML/System.hpp"

#include <functional>

#include "nlohmann/json.hpp"

namespace GUI
{

    class Button : public Base
    {
    public:
        /// @brief Constructor
        /// @param cfg button config
        Button(nlohmann::json &cfg);
        /// @brief Set button label
        /// @param str label text
        void set_text(std::string str);

        /// @brief Add GUI element
        /// @param ctrl pointer to GUI element
        /// @return always false, because button cannot contain elements
        bool add(Base *ctrl) override;
        /// @brief mouse press callback
        /// @param e mouse button event
        void on_press(const sf::Event::MouseButtonEvent &e) override;
        /// @brief mouse release callback
        /// @param e mouse button event
        void on_release(const sf::Event::MouseButtonEvent &e) override;
        /// @brief mouse click callback
        /// @param e mouse button event
        void on_click(const sf::Event::MouseButtonEvent &e) override;
        /// @brief cursor enter callback
        void on_enter() override;
        /// @brief cursor leave callback
        void on_leave() override;
        /// @brief Set on click callback
        /// @param callback callback to set
        void set_click_callback(std::function<void(Button &)> callback);

        void draw(sf::RenderTarget &target, const sf::RenderStates &states) const;

    private:
        sf::RectangleShape _body;
        sf::Text _text;
        std::function<void(Button &)> _on_click_callback;
        sf::Color _body_color;

        /// @brief default click callback, do nothing
        /// @param btn buttun
        static void default_on_click_callback(Button &btn);
    };

};

#endif // INCLUDE_GUI_GUI_BUTTON_HPP
