#ifndef INCLUDE_GUI_GUI_BASE_HPP
#define INCLUDE_GUI_GUI_BASE_HPP

#include "resources.hpp"
#include "SFML/Graphics.hpp"
#include "nlohmann/json.hpp"

namespace GUI
{
    class Base : public sf::Transformable, public sf::Drawable
    {
    public:
        /// @brief Constructor
        Base();
        /// @brief Constructor
        /// @param cfg element config
        /// @param res_mngr resources
        Base(nlohmann::json &cfg, const Resources::Manager &res_mngr);

        /// @brief Get element state
        /// @return true if element is fixed? else false
        bool is_fixed() const;
        /// @brief Add GUI element
        /// @param ctrl pointer to GUI element
        /// @return true if element added, else false
        virtual bool add(Base *ctrl);
        /// @brief Erase element by poonter
        /// @param ctrl pointer to erasing element
        void erase(Base *ctrl);
        /// @brief Detach element from parent
        void detach();
        /// @brief Return element parent and positon
        void retach();
        /// @brief Get element global position (regarding the window)
        /// @return global position in pixel
        sf::Vector2i get_global_position();
        /// @brief Get element id
        /// @return id string
        std::string get_id();

        /// @brief Update hower
        /// @param mouse_pos relative cursor position
        /// @param[out] hover hovered element
        /// @return true if cursor hover this or some children, else false
        virtual bool update_hover(sf::Vector2i mouse_pos, Base *&hover);
        /// @brief mouse click callback
        /// @param e mouse button event
        virtual void on_click(const sf::Event::MouseButtonEvent &e);
        /// @brief cursor enter callback
        virtual void on_enter();
        /// @brief cursor leave callback
        virtual void on_leave();
        /// @brief on focus callback
        virtual void on_focus();
        /// @brief on defocus callback
        virtual void on_defocus();
        /// @brief mouse press callback
        /// @param e mouse button event
        virtual void on_press(const sf::Event::MouseButtonEvent &e);
        /// @brief mouse release callback
        /// @param e mouse button event
        virtual void on_release(const sf::Event::MouseButtonEvent &e);
        /// @brief keyboard key press callback
        /// @param e key event
        virtual void on_key_press(const sf::Event::KeyEvent &e);
        /// @brief on drag callback
        /// @param[out] drag the pointer to the dragged element
        virtual void on_drag(Base *&drag);
        /// @brief on drop callback
        /// @param[in] hover current hovered element
        virtual void on_drop(Base *hover);
        /// @brief on text input callback
        /// @param e text event
        virtual void on_input_text(const sf::Event::TextEvent &e);
        /// @brief on mouse move callback
        /// @param e mouse move event
        virtual void on_mouse_move(const sf::Event::MouseMoveEvent &e);
        
        virtual void draw(sf::RenderTarget &target, const sf::RenderStates &states) const;

    protected:
        Base *_parent;
        Base *_old_parent;
        sf::Vector2f _old_position;

        /// @brief Set element hitbox size
        /// @param hitbox new hitbox size
        void set_hitbox(sf::Vector2f hitbox);

    private:
        std::vector<Base *> _childes;
        // local bounds (does not include position)
        sf::FloatRect _bounds;
        bool _is_fixed;
        // uniq text id for find element
        std::string _id;
    };
}
#endif // INCLUDE_GUI_GUI_BASE_HPP
