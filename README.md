# plato-shell — The Unified MUD Shell

Compose all Plato modules (puppeteer, manus, vision, sonar, correlator, tick, fleet) into a single MUD-like text-based world an agent can navigate and interact with. The central hub of the Plato ecosystem.

**Part of [SuperInstance OpenConstruct](https://github.com/SuperInstance/OpenConstruct).**

## What This Gives You

- **MUD-like rooms** — every module surface (desktop, devices, knowledge, fleet) is a navigable room
- **Command dispatch** — `look`, `go north`, `examine`, `use`, `talk` route to the appropriate module
- **Module composition** — puppeteer rooms, manus commands, vision scenes, fleet status in one world
- **ShellRoom** — title, description, exits, objects, agents present
- **ShellResponse** — text output, room transitions, events, module-specific output

## Quick Start

```rust
use plato_shell::{ShellRoom, PlatoShell};

// Rooms represent module surfaces
let hub = ShellRoom::new(
    "Central Hub",
    "Passages lead to puppeteer, manus, vision, and fleet rooms."
);

let vision_room = ShellRoom::new(
    "Vision Lab",
    "Camera feeds stream as text descriptions. Objects detected: person, desk, laptop."
);

// Agent navigates the unified world
let mut shell = PlatoShell::new();
let response = shell.process("look");
// → Room description with exits and objects

let response = shell.process("go vision");
// → Room change to Vision Lab
```

## How It Fits

This is the composition layer. [plato-puppeteer](https://github.com/SuperInstance/plato-puppeteer) provides desktop rooms, [plato-manus](https://github.com/SuperInstance/plato-manus) provides file/device commands, [plato-vision](https://github.com/SuperInstance/plato-vision) and [plato-sonar-text](https://github.com/SuperInstance/plato-sonar-text) provide sensory descriptions, [plato-fleet](https://github.com/SuperInstance/plato-fleet) provides device rooms, [plato-correlator](https://github.com/SuperInstance/plato-correlator) fuses events, [plato-tick](https://github.com/SuperInstance/plato-tick) routes inter-agent messages — all composed into the unified shell.

## Installation

```toml
[dependencies]
plato-shell = "0.1"
```

## License

MIT
