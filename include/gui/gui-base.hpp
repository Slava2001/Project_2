#ifndef INCLUDE_GUI_GUI_BASE_HPP
#define INCLUDE_GUI_GUI_BASE_HPP

#include "SFML/Graphics.hpp"
namespace GUI
{
    class Base : public sf::Transformable, public sf::Drawable
    {
    public:
        Base(sf::Vector2f hitbox = sf::Vector2f(0, 0), bool is_fixed = true);
        virtual void update(sf::Vector2i mose_pos);

        bool is_fixed() const;
        virtual bool add(Base *ctrl);
        void remove(Base *ctrl);
        void detach();
        void retach();
        sf::Vector2i get_global_position();

        virtual bool update_hover(sf::Vector2i mouse_pos, Base *&hover);
        virtual void on_click();
        virtual void on_enter();
        virtual void on_leave();
        virtual void on_focus();
        virtual void on_defocus();
        virtual void on_press();
        virtual void on_release();

        virtual void draw(sf::RenderTarget &target, const sf::RenderStates &states) const;

    private:
        Base *_parent;
        Base *_old_parent;
        sf::Vector2f _old_position;
        std::vector<Base *> _childes;

        sf::FloatRect _bounds;
        bool _is_fixed;
    };
}
#endif // INCLUDE_GUI_GUI_BASE_HPP
