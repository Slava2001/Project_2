#include "settings.hpp"

namespace Settings
{
    namespace Window
    {
        int height = 400;
        int width = 400;
        int fps_limit = 65;
        const char *title = "Project 2";
    }
    namespace Screen
    {
        uint32_t background_color = 0x000000ff;
    }
    namespace Text
    {
        int debug_text_size = 12;
    }
    namespace Debug
    {
        float fps_update_periud = 0.1;
    }
}
