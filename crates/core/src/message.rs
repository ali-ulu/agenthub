#[derive(Debug, Clone)]
pub enum MessageType {
    Message,
    Task,
    Result,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub id: String,
    pub message_type: MessageType,
    pub room: String,
    pub sender: String,
    pub target: Option<String>,
    pub in_reply_to: Option<String>,
    pub content: String,
}
