#include "gui-textbox.hpp"
#include "resources.hpp"

#include "SFML/Window.hpp"

using namespace GUI;

constexpr sf::Color Textbox::_defocus_color = sf::Color(200, 200, 200);
constexpr sf::Color Textbox::_focus_color = sf::Color::White;
constexpr sf::Color Textbox::_text_color = sf::Color::Black;
constexpr int Textbox::_outline_thickness = 2;
constexpr sf::Color Textbox::_outline_thickness_color = sf::Color(100, 100, 100);

Textbox::Textbox(sf::Vector2f size) : Base(size)
{
    _text_render.setString(_text);
    _text_render.setFont(Resources::Fonts::arial);
    _text_render.setFillColor(sf::Color::Black);
    _text_render.setCharacterSize(size.y);
    _body.setSize(size);
    _body.setFillColor(_defocus_color);
    _body.setOutlineThickness(_outline_thickness);
    _body.setOutlineColor(_outline_thickness_color);
}

static const char key_to_char[2][sf::Keyboard::KeyCount] = {
    {'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
     'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
     'Y', 'Z', ')', '!', '@', '#', '$', '%', '^', '&', '*', '(',
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, '{', '}',
     ':', '<', '>', '\"', '?', '|', '~', '+', '_', ' ', 0, 0,
     0, 0, 0, 0, 0, 0, 0, '+', '-', '*', '/', 0,
     0, 0, 0, '0', '1', '2', '3', '4', '5', '6', '7', '8',
     '9', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0},
    {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
     'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x',
     'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, '[', ']',
     ';', ',', '.', '\'', '/', '\\', '~', '=', '-', ' ', 0, 0,
     0, 0, 0, 0, 0, 0, 0, '+', '-', '*', '/', 0,
     0, 0, 0, '0', '1', '2', '3', '4', '5', '6', '7', '8',
     '9', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0}};

void Textbox::on_key_press(sf::Event::KeyEvent &e)
{
    int upper = e.shift ? 0 : 1;
    if (e.code == sf::Keyboard::Backspace && _text.size() > 0)
    {
        _text.pop_back();
    }
    else if (e.code >= 0 && key_to_char[upper][e.code])
    {
        _text += key_to_char[upper][e.code];
    }

    _text_render.setString(_text);
    if (_text_render.getLocalBounds().getSize().x >= _body.getSize().x)
    {
        if (_text.size() > 0)
        {
            _text.pop_back();
        }
        _text_render.setString(_text);
    }
}

void Textbox::on_focus()
{
    _body.setFillColor(_focus_color);
}

void Textbox::on_defocus()
{
    _body.setFillColor(_defocus_color);
}

bool Textbox::add(Base *ctrl)
{
    return false;
}

std::string Textbox::get_text()
{
    return _text;
}

void Textbox::draw(sf::RenderTarget &target, const sf::RenderStates &states) const
{
    sf::RenderStates states_copy(states.transform * getTransform());
    target.draw(_body, states_copy);
    target.draw(_text_render, states_copy);
}
