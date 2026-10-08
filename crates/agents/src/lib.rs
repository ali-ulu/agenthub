use agenthub_core::agent::Agent;

pub struct AgentRegistry {
    agents: Vec<Agent>,
}

impl AgentRegistry {
    pub fn new() -> Self {
        Self { agents: Vec::new() }
    }

    pub fn register(&mut self, agent: Agent) {
        self.agents.push(agent);
    }

    pub fn list(&self) -> &[Agent] {
        &self.agents
    }
}
