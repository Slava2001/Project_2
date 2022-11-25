#include "scene-manager.hpp"
#include "debug-drawer.hpp"
#include "scene-debug.hpp"

using namespace Scene;

std::unique_ptr<Base> Manager::create_scene(scene_ids id)
{
    switch (id)
    {
    case scene_ids::DEBUG:
        return std::make_unique<Debug_scene>(*this);
    }
    throw std::out_of_range("Wrong scene id");
}

Manager::Manager()
{
    _requaer_load_scene = false;
    _scene_id = scene_ids::DEBUG;
}

void Manager::update()
{
    if (_scene)
    {
        _scene->update();
    }
    if (_requaer_load_scene)
    {
        change_scene();
    }
}

void Manager::event_handling(const sf::Event &e)
{
    if (_scene)
    {
        _scene->event_handling(e);
    }
    if (_requaer_load_scene)
    {
        change_scene();
    }
}

void Manager::load_scene(scene_ids id)
{
    _scene_id = id;
    _requaer_load_scene = true;
}

void Manager::change_scene()
{
    if (_requaer_load_scene)
    {
        _requaer_load_scene = false;
        _scene.reset();
        _scene = create_scene(_scene_id);
    }
}

void Manager::draw(sf::RenderTarget &target, const sf::RenderStates &states) const
{
    if (_scene)
    {
        target.draw(*_scene);
    }
}
