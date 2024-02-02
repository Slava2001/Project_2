#include "lan-channel.hpp"
#define LOG_LVL LOG_LVL_INFO
#include "logger.hpp"

#include <cstring>

using namespace Lan;

Recv_channel::Recv_channel():
    _status(Status::OK)
{

}

Status Recv_channel::recv(struct Packet &packet)
{
    if (_status != Status::OK) {
        return _status;
    }
    if (_recv_buff.empty()) {
        return Status::NOT_READY;
    }
    packet = _recv_buff.front();
    _recv_buff.pop();
    return Status::OK;
}

Status Recv_channel::get_status()
{
    return _status;
}

void Recv_channel::reset()
{

}

const sf::IpAddress& Recv_channel::get_addr() const
{
    return _send_buff.front().get_sender_addr();
}

uint16_t Recv_channel::get_port() const
{
    return _send_buff.front().get_sender_port();
}

bool Recv_channel::has_packet_to_send()
{
    return !_send_buff.empty();
}

Packet* Recv_channel::get_packet()
{
    return &_send_buff.front();
}

void Recv_channel::pop_packet()
{
    _send_buff.pop();
}

Status Recv_channel::send_not_important(const struct Packet &packet)
{
    if (_send_buff.size() >= packet_buffer_max_len) {
        log_warn("Send queue is full");
        return Status::NOT_READY;
    }
    _send_buff.push(packet);
    return Status::OK;
}

void Recv_channel::send_ack(const Packet *packet)
{
    Packet ansv(Packet::CHANNEL_TAG_ACK);
    ansv << packet->get_sequence_counter();
    ansv.set_sender_addr(packet->get_sender_addr());
    ansv.set_sender_port(packet->get_sender_port());
    log_debug("Send ACK for ", packet->get_sequence_counter());
    if (send_not_important(ansv) != Status::OK) {
        log_error("Failed to send ACK on packet: ", packet->get_sequence_counter());
    }
}

Status Recv_channel::take_packet(Packet *packet, sf::IpAddress addr, uint16_t port)
{
    packet->set_sender_addr(addr);
    packet->set_sender_port(port);

    if (packet->is_important()) {
        send_ack(packet);
    }

    if (packet->get_tag() == Packet::CHANNEL_TAG_ACK ||
        packet->get_tag() == Packet::CHANNEL_TAG_PING) {
        return Status::OK;
    }

    if (_recv_buff.size() >= packet_buffer_max_len) {
        log_warn("Recv buffer is full. Drop packet from ", addr.toString(), ":", port);
        if (packet->is_important()) {
            log_error("Droped important packet");
        }
        return Status::NOT_READY;
    }

    _recv_buff.push(*packet);
    return Status::OK;
}
