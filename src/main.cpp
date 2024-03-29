#include "settings.hpp"
#include "debug-drawer.hpp"
#define LOG_LVL LOG_LVL_DEBUG
#include "logger.hpp"
#include "scene-manager.hpp"
#include "util.hpp"

#include "SFML/Graphics.hpp"
#include "SFML/System.hpp"

#include <fstream>
#include <sstream>

int main()
{
    log_enter();
    // disable SFML logs
    sf::err().rdbuf(nullptr);
    time_t seed = time(nullptr);
    log_info("Start. Seed: ", seed);
    srand(time(nullptr));

    sf::RenderWindow window(sf::VideoMode(sf::Vector2u(Settings.window.width,
                                                       Settings.window.height)),
                            Settings.window.title);
    window.setFramerateLimit(Settings.window.fps_limit);

    nlohmann::json cfg = cfg_from_file("./resources/cfg.json");

    if (cfg["debug"].is_null()) {
        log_fatal("Debug config not provided");
        throw std::runtime_error("Debug config not provided");
    }
    Debug_drawer debug_drawer(cfg["debug"]);

    if (!cfg["main_scene"].is_string()) {
        log_fatal("Main scene path to config not provided");
        throw std::runtime_error("Main scene path to config not provided");
    }
    nlohmann::json main_scene_cfg = cfg_from_file(cfg["main_scene"]);
    Scene::Manager mgr;
    mgr.load_scene(main_scene_cfg);

    sf::Clock fps_clock;
    int frame_counter = 0;
    int current_fps = 0;
    while (window.isOpen()) {
        sf::Event event;
        while (window.pollEvent(event)) {
            if (event.type == sf::Event::Closed) {
                window.close();
            }
            mgr.event_handling(event);
        }

        frame_counter++;
        if (fps_clock.getElapsedTime().asSeconds() > Settings.debug.fps_update_periud_s) {
            current_fps = frame_counter / fps_clock.restart().asSeconds();
            frame_counter = 0;
        }
        Debug_drawer::add_string("FPS: ", current_fps);

        mgr.update(1./Settings.window.fps_limit);

        window.clear(sf::Color(0xaaaacc));
        window.draw(mgr);
        window.draw(debug_drawer);
        window.display();
    }
    return 0;
}
