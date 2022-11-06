#include "gui-textbox.hpp"
#include "resources.hpp"

#include "SFML/Window.hpp"

using namespace GUI;

Textbox::Textbox(sf::Vector2f size) : Base(size)
{
    _text_render.setFont(Resources::Fonts::arial);
    _text_render.setFillColor(sf::Color::Black);
    _text_render.setCharacterSize(size.y);
    _body.setSize(size);
    _body.setFillColor(_defocus_color);
    _body.setOutlineThickness(_outline_thickness);
    _body.setOutlineColor(_outline_thickness_color);
    _in_focus = false;
    _is_presed = false;
}

static const char key_to_char[2][sf::Keyboard::Escape] = {
    {'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
     'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
     'Y', 'Z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'},
    {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
     'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x',
     'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'}};

void Textbox::update()
{
    if (!_in_focus)
    {
        return;
    }
    if (_is_presed)
    {
        if (sf::Keyboard::isKeyPressed(_presed_key))
        {
            return;
        }
        _is_presed = false;
    }

    int upper = sf::Keyboard::isKeyPressed(sf::Keyboard::LShift) ? 0 : 1;
    if (sf::Keyboard::isKeyPressed(sf::Keyboard::Backspace))
    {
        if (_text.size() > 0)
        {
            _text.pop_back();
        }
        _is_presed = true;
        _presed_key = sf::Keyboard::Key::Backspace;
    }
    else if (sf::Keyboard::isKeyPressed(sf::Keyboard::Space))
    {
        _text += " ";
        _is_presed = true;
        _presed_key = sf::Keyboard::Key::Space;
    }
    else
    {
        for (int i = 0; i < sf::Keyboard::Escape; i++)
        {
            if (sf::Keyboard::isKeyPressed((sf::Keyboard::Key)i))
            {
                _text += key_to_char[upper][i];
                _is_presed = true;
                _presed_key = (sf::Keyboard::Key)i;
                break;
            }
        }
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
    _in_focus = true;
    _body.setFillColor(_focus_color);
}

void Textbox::on_defocus()
{
    _in_focus = false;
    _body.setFillColor(_defocus_color);
}

bool Textbox::add(Base *ctrl)
{
    return false;
}

std::string Textbox::text()
{
    return _text;
}

void Textbox::draw(sf::RenderTarget &target, const sf::RenderStates &states) const
{
    sf::RenderStates states_copy(states.transform * getTransform());
    target.draw(_body, states_copy);
    target.draw(_text_render, states_copy);
}
