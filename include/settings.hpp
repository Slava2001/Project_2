#ifndef INCLUDE_GLOBAL_SETTINGS_HPP
#define INCLUDE_GLOBAL_SETTINGS_HPP

#include "SFML/Graphics.hpp"
#include <string>

struct Settings {
    std::string path_to_settings;
    struct Window {
        int height;
        int width;
        int fps_limit;
        std::string title;
    } window;
    struct Text {
        struct Fonts_path {
            std::string main;
        } fonts_path;
        int debug_text_size;
    } text;
    struct Debug {
        float fps_update_periud_s;
    } debug;
    struct GUI_cfg_path {
        std::string main;
    } gui_cfg_path;

    /// @brief Load settings
    void load();
    /// @brief Save settings
    void save();
};

extern struct Settings Settings;

#endif // INCLUDE_GLOBAL_SETTINGS_HPP
