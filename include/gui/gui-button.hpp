#ifndef INCLUDE_GUI_GUI_BUTTON_HPP
#define INCLUDE_GUI_GUI_BUTTON_HPP

#include "gui-base.hpp"
#include "SFML/System.hpp"

namespace GUI
{
    class Button : public Base
    {
    public:
        Button();
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
    };
};

#endif // INCLUDE_GUI_GUI_BUTTON_HPP
