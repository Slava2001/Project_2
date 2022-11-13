#ifndef INCLUDE_GUI_GUI_TEXTBOX_HPP
#define INCLUDE_GUI_GUI_TEXTBOX_HPP

#include "gui-base.hpp"
#include "SFML/System.hpp"

namespace GUI
{
    class Textbox : public Base
    {
    public:
        /// @brief Constructor
        /// @param size line size
        Textbox(float len = 100, int char_size = 16, int line_count = 1);
        /// @brief Get text from textbox
        /// @return textbox text
        std::string get_text();

        /// @brief Add GUI element
        /// @param ctrl pointer to GUI element
        /// @return true if element added, else false
        bool add(Base *ctrl) override;
        /// @brief on focus callback
        void on_focus() override;
        /// @brief on defocus callback
        void on_defocus() override;
        /// @brief keyboard key press callback
        /// @param e key event
        void on_key_press(sf::Event::KeyEvent &e) override;

        void draw(sf::RenderTarget &target, const sf::RenderStates &states) const;

    private:
        static const sf::Color _defocus_color;
        static const sf::Color _focus_color;
        static const sf::Color _text_color;
        static const int _outline_thickness;
        static const sf::Color _outline_thickness_color;

        sf::RectangleShape _body;
        sf::Text _text_render;
        std::string _text;

        float _line_spasing;

        void push_char(char c);
        void pop_char();
    };
};
#endif // INCLUDE_GUI_GUI_TEXTBOX_HPP
