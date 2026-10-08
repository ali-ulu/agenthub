pub trait ProcessManager {
    fn start(&self, command: &str) -> std::io::Result<()>;
    fn stop(&self, agent_id: &str) -> std::io::Result<()>;
}
