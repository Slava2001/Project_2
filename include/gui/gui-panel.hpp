#ifndef INCLUDE_GUI_GUI_PANEL_HPP
#define INCLUDE_GUI_GUI_PANEL_HPP

#include "gui-base.hpp"
#include "gui-manager.hpp"
#include "SFML/System.hpp"

namespace GUI
{
    class Panel : public Base
    {
    public:
        Panel();
        void on_enter();
        void on_leave();

        virtual void draw(sf::RenderTarget &target, const sf::RenderStates &states) const;

    private:
        sf::RectangleShape _head;
        sf::RectangleShape _body;
    };
};
#endif // INCLUDE_GUI_GUI_PANEL_HPP
