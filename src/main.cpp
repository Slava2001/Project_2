#include "settings.hpp"
#include "resources.hpp"
#include "debug-drawer.hpp"
#include "gui-manager.hpp"

#include "SFML/Graphics.hpp"
#include "SFML/System.hpp"

#include <sstream>

int main()
{
    srand(time(nullptr));
    Resources.load();

    sf::RenderWindow window(sf::VideoMode(sf::Vector2u(Settings.window.width,
                                                       Settings.window.height)),
                            Settings.window.title);
    window.setFramerateLimit(Settings.window.fps_limit);

    Debug_drawer debug_drawer;

    sf::Clock clock;
    int frame_counter = 0;
    int current_fps = 0;

    GUI::Manager gui(Settings.gui_cfg_path.main);
    
    uint8_t r = 0, g = 0, b = 0;
    GUI::Slider *slider_r = gui.get_elem<GUI::Slider>("background_color_r");
    GUI::Slider *slider_g = gui.get_elem<GUI::Slider>("background_color_g");
    GUI::Slider *slider_b = gui.get_elem<GUI::Slider>("background_color_b");
    GUI::Button *button = gui.get_elem<GUI::Button>("press_me_button");
    GUI::Textbox *textbox = gui.get_elem<GUI::Textbox>("command_text_box");

    slider_r->set_change_value_callback([&](GUI::Slider &s) { 
        r = s.get_value(); 
        textbox->clear();
        *textbox << "color: (" << (int)r << ", " << (int)g << ", " << (int)b << ")";
    });
    slider_g->set_change_value_callback([&](GUI::Slider &s) { 
        g = s.get_value(); 
        textbox->clear();
        *textbox << "color: (" << (int)r << ", " << (int)g << ", " << (int)b << ")";
    });
    slider_b->set_change_value_callback([&](GUI::Slider &s) { 
        b = s.get_value(); 
        textbox->clear();
        *textbox << "color: (" << (int)r << ", " << (int)g << ", " << (int)b << ")";
    });
    button->set_click_callback([&](GUI::Button &btn) { 
        slider_r->set_value(r = rand() % 256); 
        slider_g->set_value(g = rand() % 256); 
        slider_b->set_value(b = rand() % 256); 
    });

    while (window.isOpen())
    {
        sf::Event event;
        while (window.pollEvent(event))
        {
            if (event.type == sf::Event::Closed)
            {
                window.close();
            }

            gui.event_handling(event);
        }

        frame_counter++;
        if (clock.getElapsedTime().asSeconds() > Settings.debug.fps_update_periud_s)
        {
            current_fps = frame_counter / clock.restart().asSeconds();
            frame_counter = 0;
        }

        Debug_drawer::add_string("FPS: ", current_fps);

        window.clear(sf::Color(r,g,b));
        window.draw(gui);
        window.draw(debug_drawer);
        window.display();
    }
    return 0;
}
