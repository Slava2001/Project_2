#ifndef INCLUDE_SCENE_SCENE_DEBUG_HPP
#define INCLUDE_SCENE_SCENE_DEBUG_HPP

#include "SFML/Graphics.hpp"
#include "scene-base.hpp"
#include "gui-manager.hpp"
#include "gui-manager.hpp"

namespace Scene
{
    /// @brief test scene
    class Debug_scene : public Base
    {
    public:
        /// @brief Constructor
        /// @param cfg scene json config
        Debug_scene(nlohmann::json &cfg);
        /// @brief Update scene
        /// @param delta_time time delta
        void update(float delta_time) override;
        /// @brief Handling event
        /// @param e event
        void event_handling(const sf::Event &e) override;

        void draw(sf::RenderTarget &target, const sf::RenderStates &states) const;
    private:
        GUI::Manager _gui;
    };

}

#endif // INCLUDE_SCENE_SCENE_DEBUG_HPP
