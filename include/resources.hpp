#ifndef INCLUDE_RESOUECES_HPP
#define INCLUDE_RESOUECES_HPP

#include "SFML/Graphics.hpp"
#include "nlohmann/json.hpp"

#include <map>
#include <memory>

#define DEFAULT_RESOURCE_NAME "default"

namespace Resources 
{
    class Manager
    {
    public:

        /// @brief Create resources manager
        Manager();
        /// @brief Load resources by json config 
        /// @param cfg json config
        void load(nlohmann::json &cfg);
        /// @brief Get texture by name
        /// @param name texture name
        /// @return poiner to a texture with the specified name or 
        ///         a default texture if the requested texture could not be found
        sf::Texture *get_texture(const std::string &name) const;
        /// @brief Get font by name
        /// @param name font name
        /// @return poiner to a font with the specified name or 
        ///         a default font if the requested font could not be found 
        sf::Font *get_font(const std::string &name) const;

    private:
        std::map<std::string, std::unique_ptr<sf::Texture>> _textures;
        std::map<std::string, std::unique_ptr<sf::Font>> _fonts;

        /// @brief Load resource
        /// @param type resource type
        /// @param name resource name
        /// @param path resource path
        void load_resource(const std::string &type, 
                           const std::string &name, 
                           const std::string &path);

    };
}
#endif // INCLUDE_RESOUECES_HPP 