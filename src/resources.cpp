#include "resources.hpp"
#include "settings.hpp"
#include "logger.hpp"
#include "nlohmann/json.hpp"

#include <exception>

using namespace Resources;

Manager::Manager()
{
}

void Manager::load(nlohmann::json &cfg) 
{
    if (!cfg.is_array()) {
        log_fatal("Invalid resources config type");
        throw std::runtime_error("invalid resources config");
    }

    for (const auto& res_cfg: cfg) {
        if (!res_cfg["type"].is_string()) {
            log_error("Failed to load resources. Resource type does not specified");
            continue;        
        }
        if (!res_cfg["name"].is_string()) {
            log_error("Failed to load resources. Resource name does not specified");
            continue;        
        }
        if (!res_cfg["path"].is_string()) {
            log_error("Failed to load resources. Resource path does not specified");
            continue;        
        }
        load_resource(res_cfg["type"], res_cfg["name"], res_cfg["path"]);
    }
}

void Manager::load_resource(const std::string &type, 
                            const std::string &name, 
                            const std::string &path)
{
    if (type == "texture") {
        auto t = std::make_unique<sf::Texture>();
        if (!t->loadFromFile(path)) {
            log_error("Failed to load texture. name: ", name, " path: ", path);
            return;
        }
        _textures[name] = std::move(t);
    } if (type == "font") {
        auto f = std::make_unique<sf::Font>();
        if (!f->loadFromFile(path)) {
            log_error("Failed to load font. name: ", name, " path: ", path);
            return;
        }
        _fonts[name] = std::move(f);
    } else {
        log_error("Failed to load resource, unexpected type: ", type);        
    }
    log_info("Load resource: type: ", type, " name: ", name, " path: ", path);
}

sf::Texture *Manager::get_texture(const std::string &name) const
{
    auto t = _textures.find(name);
    if (t != _textures.end()) {
        return t->second.get();
    } 
    log_warn("Texture \"", name, "\" not found");
    t = _textures.find(DEFAULT_RESOURCE_NAME);
    if (t != _textures.end()) {
        return t->second.get();
    }
    log_fatal("Default texture not found");
    throw std::runtime_error("Default texture not found");
}

sf::Font *Manager::get_font(const std::string &name) const
{
    auto t = _fonts.find(name);
    if (t != _fonts.end()) {
        return t->second.get();
    } 
    log_warn("Font \"", name, "\" not found");
    t = _fonts.find(DEFAULT_RESOURCE_NAME);
    if (t != _fonts.end()) {
        return t->second.get();
    }
    log_fatal("Default font not found");
    throw std::runtime_error("Default font not found");
}