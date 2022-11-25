#ifndef INCLUDE_SCENE_SCENE_BASE_HPP
#define INCLUDE_SCENE_SCENE_BASE_HPP

#include "SFML/Graphics.hpp"
#include "nlohmann/json.hpp"

namespace Scene
{
    /// @brief Base scene class
    class Base : public sf::Drawable
    {
    public:
        /// @brief Scene
        /// @param cfg scene json config
        Base(nlohmann::json &cfg);
        /// @brief Update scene
        /// @param delta_time time delta
        virtual void update(float delta_time) = 0;
        /// @brief Handling event
        /// @param e event
        virtual void event_handling(const sf::Event &e) = 0;

        virtual void draw(sf::RenderTarget &target, const sf::RenderStates &states) const = 0;

    private:
    };
}

#endif // INCLUDE_SCENE_SCENE_BASE_HPP
