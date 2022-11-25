#include "scene-base.hpp"
#include "scene-manager.hpp"

using namespace Scene;

Base::Base(Manager &mgr) : _manager(mgr)
{
}

void Base::load_scene(scene_ids id)
{
    _manager.load_scene(id);
}
