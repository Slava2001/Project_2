#include "resources.hpp"
#include "settings.hpp"
#include "nlohmann/json.hpp"

#include <exception>

using namespace Resources;

Manager::Manager(const nlohmann::json &cfg)
{
    if (!cfg.is_array()) {
        throw std::runtime_error("invalid resources config");
    }

    for (const auto& res_cfg: cfg) {
        std::string type = res_cfg.value("type", "");
        std::string name = res_cfg.value("name", "");
        std::string path = res_cfg.value("name", "");
    }

}

sf::Texture *Manager::get_texture(const std::string &name)
{

}

sf::Texture *Manager::get_font(const std::string &name)
{

}