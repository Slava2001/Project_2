#ifndef INCLUDE_SCENE_SCENE_DEBUG_HPP
#define INCLUDE_SCENE_SCENE_DEBUG_HPP

#include "SFML/Graphics.hpp"
#include "scene-base.hpp"
#include "gui-manager.hpp"

namespace Scene
{
    class Debug_scene : public Base
    {
    public:
        /// @brief Constructor
        /// @param mgr scene manager
        Debug_scene(Manager &mgr);
        /// @brief Update scene
        void update() override;
        /// @brief Handling event
        /// @param e event
        void event_handling(const sf::Event &e) override;

        void draw(sf::RenderTarget &target, const sf::RenderStates &states) const;

    private:
        GUI::Manager _gui;
        GUI::Panel _panel_1;
        GUI::Textbox _tb_output;
        GUI::Panel _panel_2;
        GUI::Textbox _tb;
        GUI::Button _btn;
        GUI::Slider _slider;
    };

}

#endif // INCLUDE_SCENE_SCENE_DEBUG_HPP
