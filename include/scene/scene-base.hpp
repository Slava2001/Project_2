#ifndef INCLUDE_SCENE_SCENE_BASE_HPP
#define INCLUDE_SCENE_SCENE_BASE_HPP

#include "SFML/Graphics.hpp"

namespace Scene
{
    class Manager;

    enum class scene_ids
    {
        DEBUG
    };

    class Base : public sf::Drawable
    {
    public:
        /// @brief Scene
        /// @param mgr scene manager
        Base(Manager &mgr);
        /// @brief Update scene
        virtual void update() = 0;
        /// @brief Handling event
        /// @param e event
        virtual void event_handling(const sf::Event &e) = 0;

        virtual void draw(sf::RenderTarget &target, const sf::RenderStates &states) const = 0;

    protected:
        /// @brief Load scene by id
        /// @param id scene id
        void load_scene(scene_ids id);

    private:
        friend class Manager;
        Manager &_manager;
    };
}

#endif // INCLUDE_SCENE_SCENE_BASE_HPP
