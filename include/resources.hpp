#ifndef INCLUDE_RESOUECES_HPP
#define INCLUDE_RESOUECES_HPP

#include "SFML/Graphics.hpp"
#include "nlohmann/json.hpp"

#include <map>
#include <memory>

namespace Resources 
{
    class Manager
    {
    public:

        /// @brief Create resources manager form json config
        /// @param cfg json config
        Manager(const nlohmann::json &cfg);
        /// @brief Get texture by name
        /// @param name texture name
        /// @return poiner to a texture with the specified name or 
        ///         a default texture if the requested texture could not be found 
        ///         (nullptr if the default texture is also not specified) 
        sf::Texture *get_texture(const std::string &name);
        /// @brief Get font by name
        /// @param name font name
        /// @return poiner to a font with the specified name or 
        ///         a default font if the requested font could not be found 
        ///         (nullptr if the default font is also not specified) 
        sf::Texture *get_font(const std::string &name);

    private:
        std::map<std::string, std::unique_ptr<sf::Texture>> _textures;
        std::map<std::string, std::unique_ptr<sf::Font>> _fonts;
    };
}
#endif // INCLUDE_RESOUECES_HPP
