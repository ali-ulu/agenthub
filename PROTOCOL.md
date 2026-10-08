# AgentHub Protocol

AgentHub uses a small normalized message model so different agent runtimes do not need to understand each other's native protocols.

## Message

```json
{
  "type": "message",
  "room": "main",
  "sender": "claude",
  "content": "I found a problem in auth.ts"
}
```

## Task

```json
{
  "type": "task",
  "room": "main",
  "sender": "claude",
  "target": "codex",
  "content": "Review auth.ts"
}
```

## Result

```json
{
  "type": "result",
  "room": "main",
  "sender": "codex",
  "in_reply_to": "task-123",
  "content": "I found two issues..."
}
```

## Routing

The first router should support:

- `@claude`
- `@codex`
- `@gemini`
- `@all`

The protocol should remain transport-agnostic so WebSocket, Tauri IPC and a future local socket can share the same message model.
