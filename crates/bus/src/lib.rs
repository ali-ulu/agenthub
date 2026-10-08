use agenthub_core::message::Message;

pub trait MessageBus {
    fn publish(&self, message: Message);
}
