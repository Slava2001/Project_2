#include "lan-client.hpp"
#include "lan-object.hpp"
#include "logger.hpp"

using namespace Lan;

Client::Client(sf::IpAddress addr, uint16_t port):
    _channel(nullptr),
    _is_connected(false),
    _server_addr(addr),
    _server_port(port)
{
    _timer.start();
    _manager.start();
}

Client::~Client()
{
    for (auto &obj_set: _objects) {
        if (!obj_set.empty()) {
            log_fatal("Trying destroy client while some tag subscriber still life");
        }
    }
    if (!_timered_objects.empty()) {
        log_fatal("Trying destroy client while some timer subscriber still life");
    }
    _manager.close(_channel);
    _manager.stop();
}

Status Client::update()
{
    _manager.update();

    if (_channel && _channel->get_status() != Status::OK) {
        Status status = _channel->get_status();
        _manager.close(_channel);
        _channel = nullptr;
        _is_connected = false;
        return status;
    }

    if (!_is_connected) {
        if (!_channel) {
            _channel = _manager.open(_server_addr, _server_port);
            Packet request_slot(Packet::TAG_REQUEST_SLOT);
            _channel->send(request_slot);
        }

        Packet recv;
        if (_channel->recv(recv) == Lan::Status::OK) {
            if (recv.get_tag() == Packet::TAG_CONFIRM_CONNECT) {
                _is_connected = true;
            }
        }
        return Status::NOT_READY;
    }

    if (_channel->get_status() == Status::OK) {
        Packet recv;
        if (_channel->recv(recv) == Lan::Status::OK) {
            for (const auto &o: _objects.at(recv.get_tag())) {
                o->recv(recv);
            }
        }
    }

    if (_timer.getElapsedTime() >= TICK_TIME) {
        _timer.restart();
        for (const auto &obj: _timered_objects) {
            obj->on_tick();
        }
    }

    return Status::OK;
}

Status Client::send(const Packet &packet)
{
    if (_channel) {
        return _channel->send(packet);
    }
    return Status::NOT_READY;
}

void Client::subscribe(Object *object, Packet::Tag tag)
{
    _objects.at(tag).insert(object);
}

void Client::unsubscribe(Object *object)
{
    for (auto &obj_set: _objects) {
        obj_set.erase(object);
    }
    _timered_objects.erase(object);
}

void Client::subscribe_on_timer(Object *object)
{
    _timered_objects.insert(object);
}
