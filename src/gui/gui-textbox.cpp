#include "gui-textbox.hpp"
#include "resources.hpp"
#include "debug-drawer.hpp"
#include "util.hpp"

#include "SFML/Window.hpp"

using namespace GUI;

constexpr char Textbox::_fake_newline_marker = 13;

Textbox::Textbox(nlohmann::json &cfg, const Resources::Manager &res_mngr):
    Base(cfg, res_mngr),
    _text_render(*res_mngr.get_font(cfg.value("font", DEFAULT_RESOURCE_NAME)))
{
    int line_count = cfg.value("line_count", 1);
    int len = cfg.value("width", 0);
    int char_size = cfg.value("font_size", 0);
    _outline_thickness = cfg.value("outline_thickness", 2);
    _defocus_color = color_from_string(cfg.value("body_color", "#000000"));
    _focus_color = color_from_string(cfg.value("focus_color", "#000000"));
    _text_color = color_from_string(cfg.value("text_color", "#000000"));
    _outline_thickness_color = color_from_string(cfg.value("outline_thickness_color", "#000000"));
    _is_scroling = cfg.value("is_scroling", false);
    _is_changeable = cfg.value("is_changeable", true);
    _is_multiline = line_count > 1;
    _text_render.setFillColor(_text_color);
    _text_render.setCharacterSize(char_size);

    _line_spasing = _text_render.getFont().getLineSpacing(char_size) *
                    _text_render.getLineSpacing();

    sf::Vector2f textbox_size(len, _line_spasing * line_count);
    set_hitbox(textbox_size);
    _body.setSize(textbox_size);
    _body.setFillColor(_defocus_color);
    _body.setOutlineThickness(_outline_thickness);
    _body.setOutlineColor(_outline_thickness_color);
}

void Textbox::on_key_press(const sf::Event::KeyEvent &e)
{
    if (_is_changeable)
    {
        if (e.code == sf::Keyboard::Key::Enter && ((e.shift && _is_multiline) || !_is_multiline))
        {
            if (_enter_callback)
            {
                _enter_callback(*this);
            }
        }
        else if (e.code == sf::Keyboard::Key::Enter)
        {
            push_char('\n');
        }
        else if (e.code == sf::Keyboard::Key::Backspace)
        {
            pop_char();
        }
    }
}

void Textbox::on_input_text(const sf::Event::TextEvent &e)
{
    char c = e.unicode < 127 ? e.unicode : '#';
    if (_is_changeable && isprint(c))
    {
        push_char(c);
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
            _text.erase(_text.size() - 2, 2);
        }
        _text_render.setString(_text);
    }
}

void Textbox::scroll()
{
    auto ptr = _text.find('\n');
    if (ptr != std::string::npos)
    {
        _text.erase(0, _text.find('\n') + 1);
    }
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
    std::size_t i = text.find(_fake_newline_marker);
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
