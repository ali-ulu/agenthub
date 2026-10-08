# AgentHub Architecture

## Goal

AgentHub is a local-first communication layer that lets the user and multiple AI coding agents communicate through one shared desktop application.

The first milestone is communication, not autonomous orchestration.

## System

```text
                         USER
                           │
                    Tauri Desktop UI
                           │
                      Rust Core
                           │
        ┌──────────────────┼──────────────────┐
        │                  │                  │
   Agent Registry       Message Bus       Room Manager
        │                  │                  │
   Process Manager    Task Router       Context (future)
        │
   ┌────┼───────────────┐
   │    │               │
Claude CLI          Codex CLI        Gemini CLI
   │                    │
   └──────────────┬─────┘
                  │
           VS Code Extension
                  │
               AgentHub
```

## Components

- **Desktop UI**: Tauri application for rooms, agents, messages and task state.
- **Rust Core**: local control plane and lifecycle manager.
- **Message Bus**: routes messages between user and agents.
- **Room Manager**: maintains shared conversation rooms.
- **Agent Registry**: tracks connected agents, capabilities and status.
- **Process Manager**: starts/stops and monitors local CLI agents.
- **PTY Manager**: provides interactive terminal streams where required.
- **Adapters**: normalize agent-specific CLI protocols into the AgentHub protocol.
- **VS Code Bridge**: connects IDE agents to the same local bus.

## Non-goals for v0.1

- Autonomous swarm planning
- Vector database / semantic memory
- Cloud service
- Agent model hosting
- Production deployment orchestration
- HUQAN integration

These can be evaluated after reliable communication exists.
