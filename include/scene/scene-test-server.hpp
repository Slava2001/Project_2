#ifndef INCLUDE_SCENE_SCENE_TEST_SERVER_HPP
#define INCLUDE_SCENE_SCENE_TEST_SERVER_HPP

#include "SFML/Graphics.hpp"
#include "scene-base.hpp"
#include "gui-manager.hpp"
#include "lan-server.hpp"

namespace Scene
{
    /// @brief test scene
    class Test_server_scene : public Base
    {
    public:
        /// @brief Constructor
        /// @param cfg scene json config
        Test_server_scene(nlohmann::json &cfg);
        /// @brief Update scene
        /// @param delta_time time delta
        void update(float delta_time) override;
        /// @brief Handling event
        /// @param e event
        void event_handling(const sf::Event &e) override;

        void draw(sf::RenderTarget &target, const sf::RenderStates &states) const;
    private:
        GUI::Manager _gui;
        Lan::Server _server;
    };

}

#endif // INCLUDE_SCENE_SCENE_TEST_SERVER_HPP
