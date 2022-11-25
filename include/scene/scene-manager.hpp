#ifndef INCLUDE_SCENE_SCENE_MANAGER_HPP
#define INCLUDE_SCENE_SCENE_MANAGER_HPP

#include "SFML/Graphics.hpp"
#include <memory>

#include "scene-base.hpp"

namespace Scene
{
    enum class scene_ids;
    class Manager : public sf::Drawable
    {
    public:
        /// @brief Constructor
        Manager();
        /// @brief Update scene
        void update();
        /// @brief Handling event
        /// @param e event
        void event_handling(const sf::Event &e);
        /// @brief Request scene loading by id.
        /// @param id scene id
        void load_scene(scene_ids id);

        void draw(sf::RenderTarget &target, const sf::RenderStates &states) const;

    private:
        std::unique_ptr<Base> _scene;

        bool _requaer_load_scene;
        scene_ids _scene_id;

        /// @brief Create scene object by id
        /// @param id scene id
        /// @return pointer to new scene
        std::unique_ptr<Base> create_scene(scene_ids id);
        /// @brief Change current scerne if it requared.
        void change_scene();
    };
}

#endif // INCLUDE_SCENE_SCENE_MANAGER_HPP
