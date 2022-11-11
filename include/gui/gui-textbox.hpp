#ifndef INCLUDE_GUI_GUI_TEXTBOX_HPP
#define INCLUDE_GUI_GUI_TEXTBOX_HPP

#include "gui-base.hpp"
#include "SFML/System.hpp"

namespace GUI
{
    class Textbox : public Base
    {
    public:
        Textbox(sf::Vector2f size = sf::Vector2f(100, 20));
        void on_focus();
        void on_defocus();
        bool add(Base *ctrl);
        void on_key_press(sf::Event::KeyEvent &e);
        std::string text();
        void draw(sf::RenderTarget &target, const sf::RenderStates &states) const;

    private:
        const sf::Color _defocus_color = sf::Color(200, 200, 200);
        const sf::Color _focus_color = sf::Color::White;
        const sf::Color _text_color = sf::Color::Black;
        const int _outline_thickness = 2;
        const sf::Color _outline_thickness_color = sf::Color(100, 100, 100);

        sf::RectangleShape _body;
        sf::Text _text_render;
        std::string _text;
    };
};
#endif // INCLUDE_GUI_GUI_TEXTBOX_HPP
