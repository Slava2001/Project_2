#ifndef INCLUDE_SCENE_SCENE_TEST_SERVER_HPP
#define INCLUDE_SCENE_SCENE_TEST_SERVER_HPP

#include "SFML/Graphics.hpp"
#include "scene-base.hpp"
#include "gui-manager.hpp"
#include "lan-server.hpp"
#include "lan-client.hpp"

namespace Scene
{
    class Chat_obj;
    /// @brief test scene
    class Test_server_scene : public Base {
    public:
        /// @brief Constructor
        /// @param cfg scene json config
        Test_server_scene(nlohmann::json &cfg);
        /// @brief Destructor
        ~Test_server_scene();
        /// @brief Update scene
        /// @param delta_time time delta
        void update(float delta_time) override;
        /// @brief Handling event
        /// @param e event
        void event_handling(const sf::Event &e) override;

        void draw(sf::RenderTarget &target, const sf::RenderStates &states) const;
    private:

        class Chat_obj: public Lan::Object {
        public:
            Chat_obj(GUI::Textbox *input, GUI::Textbox *output, Lan::Client &client);
            void recv(Lan::Packet packet) override;
            void on_tick() override;

        private:
            GUI::Textbox *_output;

        };

        GUI::Manager _gui;
        GUI::Textbox *_text_out;
        std::shared_ptr<Lan::Server> _server;
        std::shared_ptr<Lan::Client> _client;
        std::unique_ptr<Chat_obj> _obj;
    };

}

#endif // INCLUDE_SCENE_SCENE_TEST_SERVER_HPP
