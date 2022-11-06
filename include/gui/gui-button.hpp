#ifndef INCLUDE_GUI_GUI_BUTTON_HPP
#define INCLUDE_GUI_GUI_BUTTON_HPP

#include "gui-base.hpp"
#include "SFML/System.hpp"

namespace GUI
{

    class Button : public Base
    {
    public:
        Button(std::function<void(Button &)> callback = defult_on_click_callback);
        bool add(Base *ctrl);
        void on_press();
        void on_release();
        void on_enter();
        void on_leave();
        void on_click();

        void draw(sf::RenderTarget &target, const sf::RenderStates &states) const;

        void set_text(std::string str);

    private:
        sf::RectangleShape _body;
        sf::Text _text;
        std::function<void(Button &)> _on_click_callback;

        static void defult_on_click_callback(Button &btn);
    };

};

#endif // INCLUDE_GUI_GUI_BUTTON_HPP
