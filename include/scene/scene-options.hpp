#ifndef INCLUDE_SCENE_SCENE_SETTINGS_HPP
#define INCLUDE_SCENE_SCENE_SETTINGS_HPP

#include "scene-base.hpp"
#include "gui-manager.hpp"

namespace Scene
{
    class Options_scene : public Base
    {
    public:
        /// @brief Constructor
        /// @param mgr scene manager
        Options_scene(Manager &mgr);
        /// @brief Update scene
        void update() override;
        /// @brief Handling event
        /// @param e event
        void event_handling(const sf::Event &e) override;

        void draw(sf::RenderTarget &target, const sf::RenderStates &states) const;

    private:
        GUI::Manager _gui;

        GUI::Slider _background_color_slider_r;
        GUI::Slider _background_color_slider_g;
        GUI::Slider _background_color_slider_b;
        GUI::Button _load_debug_scene_button;
    };
}

#endif // INCLUDE_SCENE_SCENE_SETTINGS_HPP
