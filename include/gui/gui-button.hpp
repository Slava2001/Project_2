#ifndef INCLUDE_GUI_GUI_BUTTON_HPP
#define INCLUDE_GUI_GUI_BUTTON_HPP

#include "gui-base.hpp"
#include "SFML/System.hpp"

#include <functional>

namespace GUI
{

    class Button : public Base
    {
    public:
        /// @brief Constructor
        /// @param callback click callback
        Button(std::function<void(Button &)> callback = default_on_click_callback);
        /// @brief Set button label
        /// @param str label text
        void set_text(std::string str);

        /// @brief Add GUI element
        /// @param ctrl pointer to GUI element
        /// @return always false, because button cannot contain elements
        bool add(Base *ctrl) override;
        /// @brief mouse press callback
        /// @param e mouse button event
        virtual void on_press(const sf::Event::MouseButtonEvent &e) override;
        /// @brief mouse release callback
        /// @param e mouse button event
        virtual void on_release(const sf::Event::MouseButtonEvent &e) override;
        /// @brief mouse click callback
        /// @param e mouse button event
        virtual void on_click(const sf::Event::MouseButtonEvent &e) override;
        /// @brief cursor enter callback
        virtual void on_enter() override;
        /// @brief cursor leave callback
        virtual void on_leave() override;

        void draw(sf::RenderTarget &target, const sf::RenderStates &states) const;

    private:
        static const sf::Vector2f _size;

        sf::RectangleShape _body;
        sf::Text _text;
        std::function<void(Button &)> _on_click_callback;

        /// @brief default click callback, do nothing
        /// @param btn buttun
        static void default_on_click_callback(Button &btn);
    };

};

#endif // INCLUDE_GUI_GUI_BUTTON_HPP
