#ifndef INCLUDE_GUI_GUI_SLIDER_HPP
#define INCLUDE_GUI_GUI_SLIDER_HPP

#include "gui-base.hpp"
#include "SFML/System.hpp"

#include <functional>

namespace GUI
{
    class Slider : public Base
    {
    public:
        /// @brief Constructor
        /// @param size slider size
        /// @param min minimum value
        /// @param max maximum value
        /// @param step change value step (must be more than (max-min)/(size.x-size.y))
        /// @note the size and range must be a multiple of the step,
        ///       otherwise the slider will not be drawn exactly
        Slider(sf::Vector2f size, float min = 0, float max = 100, float step = 1);
        /// @brief Get slider value
        /// @return current value
        float get_value() const;
        /// @brief Set change value callback
        /// @param callback callback
        void set_change_value_callback(std::function<void(Slider &s)> callback);

        /// @brief mouse press callback
        /// @param e mouse button event
        void on_press(const sf::Event::MouseButtonEvent &e) override;
        /// @brief mouse release callback
        /// @param e mouse button event
        void on_release(const sf::Event::MouseButtonEvent &e) override;
        /// @brief on mouse move callback
        /// @param e mouse move event
        void on_mouse_move(const sf::Event::MouseMoveEvent &e) override;

        void draw(sf::RenderTarget &target, const sf::RenderStates &states) const;

    private:
        static const sf::Color _body_color;
        static const sf::Color _arrow_color;

        sf::RectangleShape _body;
        sf::RectangleShape _arrow;
        bool _is_buttun_press;
        sf::Vector2f _size;
        float _min;
        float _max;
        float _step;
        int _step_in_pixel;
        std::function<void(Slider &s)> _change_value_callback;

        /// @brief Update slider arrow position
        /// @param mouse_global_pos mouse global position
        void update_arrow_pos(sf::Vector2i mouse_global_pos);
    };

};

#endif // INCLUDE_GUI_GUI_SLIDER_HPP
