#ifndef INCLUDE_GUI_GUI_TEXTBOX_HPP
#define INCLUDE_GUI_GUI_TEXTBOX_HPP

#include "gui-base.hpp"
#include "SFML/System.hpp"

namespace GUI
{
    class Textbox : public Base
    {
    public:
        Textbox();
        void update(sf::Vector2i mose_pos);
        void on_focus();
        void on_defocus();
        std::string text();
        void draw(sf::RenderTarget &target, const sf::RenderStates &states) const;

    private:
        sf::RectangleShape _body;
        sf::Text _text_render;
        std::string _text;
        bool _in_focus;
        bool _is_presed;
        sf::Keyboard::Key _presed_key;
    };
};
#endif // INCLUDE_GUI_GUI_TEXTBOX_HPP
