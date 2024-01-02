#ifndef INCLUDE_GUI_GUI_TEXTBOX_HPP
#define INCLUDE_GUI_GUI_TEXTBOX_HPP

#include "resources.hpp"
#include "gui-base.hpp"
#include "SFML/System.hpp"
#include "nlohmann/json.hpp"

#include <string>
#include <sstream>
#include <functional>

namespace GUI
{
    class Textbox : public Base
    {
    public:
        /// @brief Constructor
        /// @param cfg textbox config
        /// @param res_mngr resources
        Textbox(nlohmann::json &cfg, const Resources::Manager &res_mngr);
        /// @brief Get text from textbox
        /// @return textbox text
        std::string get_text();
        /// @brief Clear textbox.
        void clear();
        /// @brief Set the scroll flag. If the flag is set and there is not enough space to add
        /// characters, the first line will be deleted
        /// @param flag flag
        void set_scroling(bool flag);
        /// @brief Set the changeable flag. If the flag is set, the user can change the contents of
        /// the textbox
        /// @param flag flag
        void set_changeable(bool flag);
        /// @brief Set Enter callback. Сalled when the user presses Enter or Enter+Shift (for a
        /// multiline textbox)
        /// @param callback callback function
        void set_enter_callback(std::function<void(Textbox &)> callback);

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
        void on_key_press(const sf::Event::KeyEvent &e) override;
        /// @brief on text input callback
        /// @param e text event
        void on_input_text(const sf::Event::TextEvent &e) override;

        void draw(sf::RenderTarget &target, const sf::RenderStates &states) const;

    private:
        static const char _fake_newline_marker;

        int _outline_thickness;
        sf::Color _defocus_color;
        sf::Color _focus_color;
        sf::Color _text_color;
        sf::Color _outline_thickness_color;
        sf::RectangleShape _body;
        sf::Text _text_render;
        std::string _text;
        float _line_spasing;
        bool _is_scroling;
        bool _is_changeable;
        bool _is_multiline;
        std::function<void(Textbox &)> _enter_callback;

        /// @brief Put char (doesn't do anything if the place is over)
        /// @param c char
        void push_char(char c);
        /// @brief Pop char (does nothing if the textbox is empty)
        void pop_char();
        /// @brief Deletes the first row by shifting all the others up
        void scroll();

        template <typename T>
        friend Textbox &operator<<(Textbox &t, T d);
    };

    template <typename T>
    Textbox &operator<<(Textbox &t, T d)
    {
        std::stringstream _sstr;
        _sstr << d;
        for (const char &c : _sstr.str())
        {
            t.push_char(c);
        }
        return t;
    }
}
#endif // INCLUDE_GUI_GUI_TEXTBOX_HPP
