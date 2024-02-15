#include "lan-object.hpp"
#include "lan-client.hpp"

using namespace Lan;

Object::Object(Client &client):
    _client(client)
{

}

Object::~Object()
{
    _client.unsubscribe(this);
}

Status Object::send(const Packet &packet)
{
    return _client.send(packet);
}

void Object::on_tick()
{
}

void Object::subscribe(Packet::Tag tag)
{
    _client.subscribe(this, tag);
}

void Object::subscribe_on_timer()
{
    _client.subscribe_on_timer(this);
}
