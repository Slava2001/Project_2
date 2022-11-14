#include "gui-textbox.hpp"
#include "resources.hpp"
#include "debug-drawer.hpp"

#include "SFML/Window.hpp"

using namespace GUI;

constexpr sf::Color Textbox::_defocus_color = sf::Color(200, 200, 200);
constexpr sf::Color Textbox::_focus_color = sf::Color::White;
constexpr sf::Color Textbox::_text_color = sf::Color::Black;
constexpr int Textbox::_outline_thickness = 2;
constexpr sf::Color Textbox::_outline_thickness_color = sf::Color(100, 100, 100);
constexpr char Textbox::_fake_newline_marker = 13;

Textbox::Textbox(float len, int char_size, int line_count)
{
    _is_scroling = false;
    _is_changeable = true;
    _is_multiline = line_count > 1;
    _text_render.setFont(Resources::Fonts::main);
    _text_render.setFillColor(sf::Color::Black);
    _text_render.setCharacterSize(char_size);

    _line_spasing = _text_render.getFont()->getLineSpacing(char_size) *
                    _text_render.getLineSpacing();

    sf::Vector2f textbox_size(len, _line_spasing * line_count);
    set_hitbox(textbox_size);
    _body.setSize(textbox_size);
    _body.setFillColor(_defocus_color);
    _body.setOutlineThickness(_outline_thickness);
    _body.setOutlineColor(_outline_thickness_color);
}

static const char key_to_char[2][sf::Keyboard::KeyCount] = {
    {'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
     'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
     'Y', 'Z', ')', '!', '@', '#', '$', '%', '^', '&', '*', '(',
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, '{', '}',
     ':', '<', '>', '\"', '?', '|', '~', '+', '_', ' ', '\n', 0,
     0, 0, 0, 0, 0, 0, 0, '+', '-', '*', '/', 0,
     0, 0, 0, '0', '1', '2', '3', '4', '5', '6', '7', '8',
     '9', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0},
    {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
     'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x',
     'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, '[', ']',
     ';', ',', '.', '\'', '/', '\\', '~', '=', '-', ' ', '\n', 0,
     0, 0, 0, 0, 0, 0, 0, '+', '-', '*', '/', 0,
     0, 0, 0, '0', '1', '2', '3', '4', '5', '6', '7', '8',
     '9', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0}};

void Textbox::on_key_press(sf::Event::KeyEvent &e)
{
    if (_is_changeable)
    {
        int upper = e.shift ? 0 : 1;
        if (e.code == sf::Keyboard::Enter && ((e.shift && _is_multiline) || !_is_multiline))
        {
            if (_enter_callback)
            {
                _enter_callback(*this);
            }
        }
        else if (e.code == sf::Keyboard::Backspace)
        {
            pop_char();
        }
        else if (e.code >= 0 && key_to_char[upper][e.code])
        {
            push_char(key_to_char[upper][e.code]);
        }
    }
}

void Textbox::push_char(char c)
{
    _text.push_back(c);
    _text_render.setString(_text);

    if (_text_render.getLocalBounds().width > _body.getSize().x)
    {
        _text.pop_back();
        _text.push_back(_fake_newline_marker);
        _text.push_back('\n');
        _text.push_back(c);
        _text_render.setString(_text);
    }

    float hight = _text_render.getLocalBounds().height;
    if (_text.size() > 0 && _text[0] == '\n')
    {
        hight += _text_render.getCharacterSize();
    }

    if (hight > _body.getSize().y)
    {
        if (_is_scroling)
        {
            scroll();
        }
        else
        {
            pop_char();
        }
        _text_render.setString(_text);
    }
}

void Textbox::pop_char()
{
    std::size_t size = _text.size();
    if (size > 0)
    {
        _text.pop_back();
        size--;
        if (size >= 2 && _text[size - 2] == _fake_newline_marker) // remove fake newline
        {
            _text.pop_back();
            _text.pop_back();
        }
        _text_render.setString(_text);
    }
}

void Textbox::scroll()
{
    _text.erase(0, _text.find('\n') + 1);
}

void Textbox::on_focus()
{
    if (_is_changeable)
    {
        _body.setFillColor(_focus_color);
    }
}

void Textbox::on_defocus()
{
    if (_is_changeable)
    {
        _body.setFillColor(_defocus_color);
    }
}

bool Textbox::add(Base *ctrl)
{
    return false;
}

std::string Textbox::get_text()
{
    std::string text(_text);
    // erase all fake newlines
    std::string::size_type i = text.find(_fake_newline_marker);
    while (i != std::string::npos)
    {
        text.erase(i, 2);
        i = text.find(_fake_newline_marker, i);
    }
    return text;
}

void Textbox::clear()
{
    _text.clear();
    _text_render.setString(_text);
}

void Textbox::set_scroling(bool flag)
{
    _is_scroling = flag;
}

void Textbox::set_changeable(bool flag)
{
    _is_changeable = flag;
}

void Textbox::set_enter_callback(std::function<void(Textbox &)> callback)
{
    _enter_callback = callback;
}

void Textbox::draw(sf::RenderTarget &target, const sf::RenderStates &states) const
{
    sf::RenderStates states_copy(states.transform * getTransform());
    target.draw(_body, states_copy);
    target.draw(_text_render, states_copy);
}
