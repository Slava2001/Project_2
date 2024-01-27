#include "settings.hpp"

#define RESOURCES_PATH "./resources/"

struct Settings Settings = {
    .path_to_settings = "./settings.json",
    .window = {
        .height = 400,
        .width = 400,
        .fps_limit = 120,
        .title = "Project 2"
    },
    .text = {
        .fonts_path = {
            .main = RESOURCES_PATH"UbuntuMono-R.ttf",
        },
        .debug_text_size = 12
    },
    .debug = {
        .fps_update_periud_s = 0.1
    },
    .gui_cfg_path = {
        .main = RESOURCES_PATH"gui_cfg_main.json"
    }
};

void Settings::load()
{
}

void Settings::save()
{
}
