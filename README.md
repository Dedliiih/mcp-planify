# MCP Planify

[![CI](https://github.com/Dedliiih/mcp-planify/actions/workflows/ci.yml/badge.svg)]()
[![License](https://img.shields.io/badge/license-MIT-blue)]()

MCP server that exposes [Planify](https://github.com/alainm23/planify) to-do lists as MCP tools. Allows any MCP client (Claude Desktop, VS Code, OpenCode, Cursor, etc.) to read and modify your tasks.

## Prerequisites

- [Planify](https://github.com/alainm23/planify) installed and running at least once
- macOS or Linux

## Installation

### 1. Download the binary

```bash
curl -LO https://github.com/Dedliiih/mcp-planify/releases/latest/download/mcp-planify
chmod +x mcp-planify
```

> Optionally, move it to your PATH so clients can find it without the full path:
>
> ```bash
> sudo mv mcp-planify /usr/local/bin/
> ```

### 2. Configure your MCP client

**OpenCode** (`~/.config/opencode/opencode.json`):
```json
{
  "mcp_config": {
    "planify": {
      "type": "local",
      "command": ["/path/to/mcp-planify"]
    }
  }
}
```

**Claude Desktop** (`claude_desktop_config.json`):
```json
{
  "mcpServers": {
    "planify": {
      "command": "/path/to/mcp-planify"
    }
  }
}
```

## Database auto-detection

By default, the server locates your Planify database at the Flatpak path:

```
~/.var/app/io.github.alainm23.planify/data/io.github.alainm23.planify/database.db
```

To point to a custom database:

```bash
export PLANIFY_DB_PATH=/custom/path/to/database.db
mcp-planify
```

## Available tools

| Tool | Description | Parameters |
|------|-------------|------------|
| `list_projects` | List all projects | — |
| `list_items` | List items with optional filters | `project_id`, `completed`, `priority` |
| `create_item` | Create a new task | `content`, `project_id`, `description`, `priority`, `due`, `labels`, `parent_id` |
| `complete_item` | Mark a task as completed | `item_id` |
| `delete_item` | Soft-delete a task | `item_id` |

## Building from source

```bash
git clone https://github.com/Dedliiih/mcp-planify
cd mcp-planify
cargo build --release
./target/release/mcp-planify
```

## Known limitations

### Planify caches items in memory

Planify loads the database on startup and keeps everything in memory. Changes made via MCP (create, complete, delete) are written to the database but **won't show in the Planify UI until you restart the app**.

This only affects the graphical UI — MCP clients always read the latest state directly from the database.

**Workaround**: restart Planify after making changes through MCP if you want to see them reflected in the app window.

## License

MIT
