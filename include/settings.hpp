#ifndef INCLUDE_GLOBAL_SETTINGS_HPP
#define INCLUDE_GLOBAL_SETTINGS_HPP

namespace Settings
{
    namespace Window
    {
        extern int height;
        extern int width;
        extern int fps_limit;
        extern const char *title;
    }
    namespace Text
    {
        extern int debug_text_size;
    }
    namespace Debug
    {
        extern float fps_update_periud;
    }
}

#endif // INCLUDE_GLOBAL_SETTINGS_HPP
