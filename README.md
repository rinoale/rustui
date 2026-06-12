# rustui

`rustui` is a small Rust TUI framework and scaffold generator for consistent
Ratatui applications. It provides reusable library modules for common
keymapping, styling, component contracts, and non-blocking event plumbing, plus
`rustui init` for generating a new editable app.

## Usage

```sh
rustui init my-tui
cd my-tui
cargo run
```

As a library during local development:

```toml
[dependencies]
rustui = { path = "../rustui" }
```

As a versioned dependency:

```toml
[dependencies]
rustui = { git = "git@github.com:rinoale/rustui.git", tag = "v0.1.0" }
```

Then import framework pieces directly:

```rust
use rustui::{
    keymap::{Key, Keymap, binding},
    runtime::spawn_event_reader,
    style::{Design, Palette, Role, style},
};
```

The generated project uses:

- Rust 2024 edition
- Ratatui 0.30.1
- Crossterm 0.29.0
- Color-Eyre 0.6.5
- Tokio 1.x

## Interface Policy

- Quit is command-mode only: `:q`, `:quit`, `:exit`, or `:q!`.
- `Esc` cancels transient UI state. It never quits.
- Bare `q` is not a global quit key.
- `Ctrl-C` is not bound to quit by default.
- `:` opens command mode from any pane.
- `?` opens help.
- `Tab` and `Shift-Tab` move focus.

## Generated Structure

- `src/app.rs`: application state, local behavior, rendering composition
- `src/tui/keymap.rs`: common key policy with editable bindings
- `src/tui/command.rs`: command-mode parser and core commands
- `src/tui/menu.rs`: footer and help menu helpers
- `src/tui/style.rs`: design tokens, selectors, and style-rule DSL
- `src/tui/theme.rs`: named themes built from element-level style rules

The scaffold is meant to be adapted. The defaults guide common behavior, but
local apps can replace bindings, add commands, or change theme roles without
forking framework internals.

## Library Structure

- `rustui::keymap`: generic key bindings over each app's local intent enum
- `rustui::style`: design tokens, typed roles, selectors, and style builder DSL
- `rustui::runtime`: Tokio-friendly terminal event pump
- `rustui::component`: lightweight component trait for shared render/input shape

## Non-Blocking UX Backbone

Generated projects use Tokio as the async runtime. The TUI runtime keeps
terminal events, rendering ticks, and application task completions separated:

- terminal input is read through an event pump, not inside application actions
- the app exposes `poll_tasks()` as the standard place to receive background
  task results
- long work should be launched with `tokio::spawn` and reported back through an
  app-owned channel
- scene transitions such as `Form -> Connecting -> Error/Form` should update
  app state instead of blocking the terminal thread

The generated sample includes a `:task` command that starts an async background
task and keeps the UI responsive while it runs.

## Design And Color Themes

In `rustui`, a color theme is not just a global palette for the whole screen.
It is a design system: named color tokens plus style rules for each visible UI
element. A small button, footer key label, focused panel border, selected list
row, command input, help overlay, warning badge, and table header should all be
addressable independently.

Generated projects include:

- `Palette`: reusable color tokens such as `surface0`, `surface1`, `text`,
  `muted`, `accent`, `success`, `warning`, and `danger`.
- `Role`: typed selectors for common TUI elements such as `PanelFocused`,
  `FooterKey`, `ButtonPrimary`, `InputFocused`, and `TableHeader`.
- `Design`: a style sheet that maps selectors to Ratatui `Style` values.
- `style()`: a small builder DSL for declaring rules, for example:

```rust
Design::new(palette)
    .role(Role::ButtonPrimary, style().fg(palette.surface0).bg(palette.accent).bold())
    .selector("deploy.confirm.button", style().fg(palette.surface0).bg(palette.danger).bold());
```

Apps can use typed roles for shared framework pieces and string selectors for
local widgets. This is closer to CSS than a palette switch: every rendered
element can have a stable style name that is easy to override, audit, and reuse.
