#ifndef INCLUDE_SCENE_SCENE_MANAGER_HPP
#define INCLUDE_SCENE_SCENE_MANAGER_HPP

#include "SFML/Graphics.hpp"
#include "nlohmann/json.hpp"
#include <memory>

#include "scene-base.hpp"
#include "scene-debug.hpp"

namespace Scene
{
    /// @brief Static class for manage scenes
    class Manager : public sf::Drawable
    {
    public:
        /// @brief Constructor
        Manager();
        /// @brief Update scene
        /// @param delta_time time delta
        void update(float delta_time);
        /// @brief Handling event
        /// @param e event
        void event_handling(const sf::Event &e);
        /// @brief Request scene loading (will be loaded on next frame)
        /// @param cfg scene json config
        static void load_scene(nlohmann::json &cfg);

        void draw(sf::RenderTarget &target, const sf::RenderStates &states) const;

    private:
        static Manager *_instance;
        std::unique_ptr<Base> _scene;

        bool _requaer_load_scene;
        nlohmann::json _scene_cfg;

        /// @brief Create scene object by id
        /// @param cfg scene json config
        /// @return pointer to new scene
        std::unique_ptr<Base> create_scene(nlohmann::json &cfg);
        /// @brief Change current scerne if it requared.
        void change_scene_if_need();
    };
}

#endif // INCLUDE_SCENE_SCENE_MANAGER_HPP
