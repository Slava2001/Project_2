#include "scene-manager.hpp"
#include "debug-drawer.hpp"
#include "logger.hpp"

using namespace Scene;

Manager *Manager::_instance = nullptr;

Manager::Manager()
{
    if (_instance) {
        log_fatal("Try create second scene manager instanse");
        throw std::runtime_error("Try create second scene manager instanse");
    }
    _instance = this;
    _requaer_load_scene = false;
}

void Manager::update(float delta_time)
{
    change_scene_if_need();
    if (_scene) {
        _scene->update(delta_time);
    }
}

void Manager::event_handling(const sf::Event &e)
{
    change_scene_if_need();
    if (_scene) {
        _scene->event_handling(e);
    }
}

void Manager::load_scene(nlohmann::json &cfg)
{
    log_info("Request scene loading");
    _instance->_scene_cfg = cfg;
    _instance->_requaer_load_scene = true;
}

void Manager::change_scene_if_need()
{
    if (_requaer_load_scene) {
        _requaer_load_scene = false;
        _scene.reset();
        _scene = create_scene(_scene_cfg);
    }
}

void Manager::draw(sf::RenderTarget &target, const sf::RenderStates &states) const
{
    if (_scene) {
        target.draw(*_scene);
    }
}

std::unique_ptr<Base> Manager::create_scene(nlohmann::json &cfg)
{
    if (!cfg["type"].is_string()) {
        log_fatal("Scene type does not specified");
        throw std::runtime_error("Scene type does not specified");
    }

    std::unique_ptr<Base> ptr;
    if (cfg["type"] == "debug") {
        ptr = std::make_unique<Debug_scene>(cfg);
    } else {
        log_fatal("Unexpected scene type: ", cfg["type"]);
        throw std::runtime_error("Unexpected scene type");
    }
    log_info("Scene loaded. type: ", cfg["type"]);
    return ptr;
}
