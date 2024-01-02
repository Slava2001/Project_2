#ifndef INCLUDE_GUI_GUI_MANAGER_HPP
#define INCLUDE_GUI_GUI_MANAGER_HPP

#include "gui-base.hpp"

#include "gui-panel.hpp"
#include "gui-textbox.hpp"
#include "gui-button.hpp"
#include "gui-slider.hpp"

#include "SFML/System.hpp"

#include "resources.hpp"
#include "nlohmann/json.hpp"

#include <string>
#include <memory>

namespace GUI
{
    class Manager : public sf::Drawable
    {
    public:
        /// @brief Constructor
        Manager();
        /// @brief Create GUI from json config
        /// @param cfg json config file
        Manager(nlohmann::json &cfg);
        /// @brief Handling event
        /// @param e event
        void event_handling(const sf::Event &e);
        /// @brief Get element by id
        /// @param id element id
        /// @return pointer to the element found
        template <typename elem_type>
        elem_type* get_elem(std::string id) {
            for (auto& e:_dynamic_elements) {
                if (e->get_id() == id) {
                    elem_type *tmp = dynamic_cast<elem_type *>(e.get());
                    if (!tmp) {
                        throw std::runtime_error("element with id " + id + " has unexpected type");
                    }
                    return tmp;
                }
            }
            throw std::runtime_error("filed to find element by id: " + id);
        }

        void draw(sf::RenderTarget &target, const sf::RenderStates &states) const;

    private:
        Base _controls;
        Base *_hover;
        Base *_drag;
        sf::Vector2i _drag_offset;
        Base *_pressed;
        bool _mouse_left_button_pressed;
        Base *_focus;
        // all dynamic allocated gui element
        std::vector<std::shared_ptr<Base>> _dynamic_elements;
        Resources::Manager _resources;

        /// @brief Update hover
        /// @param e mouse move event
        void update_hover(const sf::Event::MouseMoveEvent &e);
        /// @brief Try drag hovered element
        /// @param e mouse button event
        void drag(const sf::Event::MouseButtonEvent &e);
        /// @brief Drop drageged element
        /// @param e mouse button event
        void drop(const sf::Event::MouseButtonEvent &e);
        /// @brief Update dragged element
        /// @param e mouse move event
        void update_dragged(const sf::Event::MouseMoveEvent &e);        
        /// @brief Create gui tree by config
        /// @param ctl root element
        /// @param cfg confg
        void create_gui_tree(Base *ctl, nlohmann::json &cfg);
        /// @brief Create gui element by config
        /// @param cfg json config
        /// @return shared ptr to created element
        std::shared_ptr<Base> create_gui_element(nlohmann::json &cfg);
    };
}
#endif // INCLUDE_GUI_GUI_MANAGER_HPP
