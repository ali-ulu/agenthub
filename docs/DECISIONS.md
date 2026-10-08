# Architecture Decisions

## ADR-001: Rust core

The local control plane is Rust because AgentHub must manage local processes, PTYs, filesystem boundaries, IPC and future permissions safely.

## ADR-002: Tauri desktop

The desktop application uses Tauri so the UI remains separate from the local Rust control plane.

## ADR-003: Adapters, not agent rewrites

Claude CLI, Codex CLI, Gemini CLI and IDE agents remain independent runtimes. AgentHub normalizes communication through adapters instead of replacing them.

## ADR-004: Communication before orchestration

The first version solves the reliable shared-room problem. Autonomous swarm behavior is explicitly deferred.
