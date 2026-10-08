#[derive(Debug, Clone)]
pub struct Agent {
    pub id: String,
    pub name: String,
    pub status: AgentStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentStatus {
    Offline,
    Starting,
    Ready,
    Busy,
    Stopped,
}
