//! # Plato Shell
//!
//! The unified Plato Shell — a MUD-like text-based environment that composes
//! all Plato modules (puppeteer, manus, vision, sonar, correlator, tick, fleet)
//! into a single coherent world an agent can navigate and interact with.

use std::fmt;

// ---------------------------------------------------------------------------
// Core types
// ---------------------------------------------------------------------------

/// A room the agent can occupy — could represent a desktop, a device,
/// a Plato knowledge room, or anything a module provides.
#[derive(Clone, Debug, PartialEq)]
pub struct ShellRoom {
    pub title: String,
    pub description: String,
    pub exits: Vec<String>,
    pub objects: Vec<String>,
    pub agents_present: Vec<String>,
    pub module_source: String,
}

impl ShellRoom {
    pub fn new(title: &str, description: &str) -> Self {
        Self {
            title: title.to_string(),
            description: description.to_string(),
            exits: Vec::new(),
            objects: Vec::new(),
            agents_present: Vec::new(),
            module_source: "shell".to_string(),
        }
    }

    fn default_entry() -> Self {
        Self {
            title: "Central Hub".to_string(),
            description: "You stand in the Central Hub of Plato Shell. "
                .to_string()
                + "A soft hum of computation fills the air. "
                + "Passages lead to various module rooms.",
            exits: vec!["puppeteer".to_string(), "manus".to_string(), "vision".to_string()],
            objects: vec!["status terminal".to_string(), "help crystal".to_string()],
            agents_present: Vec::new(),
            module_source: "shell".to_string(),
        }
    }
}

impl fmt::Display for ShellRoom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "📍 {}", self.title)?;
        writeln!(f, "{}", self.description)?;
        if !self.exits.is_empty() {
            writeln!(f, "\nExits: {}", self.exits.join(", "))?;
        }
        if !self.objects.is_empty() {
            writeln!(f, "Objects: {}", self.objects.join(", "))?;
        }
        if !self.agents_present.is_empty() {
            writeln!(f, "Agents here: {}", self.agents_present.join(", "))?;
        }
        if self.module_source != "shell" {
            writeln!(f, "[{}]", self.module_source)?;
        }
        Ok(())
    }
}

/// Response returned after processing a command.
#[derive(Clone, Debug, PartialEq)]
pub struct ShellResponse {
    pub text: String,
    pub room_changed: bool,
    pub new_room: Option<ShellRoom>,
    pub events: Vec<String>,
    pub module_output: Option<String>,
}

impl ShellResponse {
    pub fn simple(text: &str) -> Self {
        Self {
            text: text.to_string(),
            room_changed: false,
            new_room: None,
            events: Vec::new(),
            module_output: None,
        }
    }

    pub fn with_room(text: &str, room: ShellRoom) -> Self {
        Self {
            text: text.to_string(),
            room_changed: true,
            new_room: Some(room),
            events: Vec::new(),
            module_output: None,
        }
    }

    pub fn with_event(mut self, event: &str) -> Self {
        self.events.push(event.to_string());
        self
    }

    pub fn with_module_output(mut self, output: &str) -> Self {
        self.module_output = Some(output.to_string());
        self
    }
}

/// Configuration for the shell — which modules are loaded, agent identity, etc.
#[derive(Clone, Debug)]
pub struct ShellConfig {
    pub loaded_modules: Vec<String>,
    pub agent_name: String,
    pub model: String,
    pub max_history: usize,
}

impl Default for ShellConfig {
    fn default() -> Self {
        Self {
            loaded_modules: Vec::new(),
            agent_name: "agent".to_string(),
            model: "default".to_string(),
            max_history: 1000,
        }
    }
}

/// Simple module trait for extensibility.
pub trait PlatoModule: std::fmt::Debug {
    fn name(&self) -> &str;
    fn handle_command(&self, command: &str, args: &str, room: &ShellRoom) -> Option<ShellResponse>;
    fn room_for(&self, exit: &str) -> Option<ShellRoom>;
}

// ---------------------------------------------------------------------------
// Built-in modules
// ---------------------------------------------------------------------------

#[derive(Debug)]
struct TickModule;

impl PlatoModule for TickModule {
    fn name(&self) -> &str {
        "tick"
    }

    fn handle_command(&self, command: &str, args: &str, _room: &ShellRoom) -> Option<ShellResponse> {
        if command == "tick" {
            let msg = if args.is_empty() { "(no message)" } else { args };
            Some(
                ShellResponse::simple(&format!("⏱️ Tick sent: {}", msg))
                    .with_event(&format!("tick:{}", msg)),
            )
        } else {
            None
        }
    }

    fn room_for(&self, _exit: &str) -> Option<ShellRoom> {
        None
    }
}

#[derive(Debug)]
struct FleetModule;

impl PlatoModule for FleetModule {
    fn name(&self) -> &str {
        "fleet"
    }

    fn handle_command(&self, command: &str, _args: &str, _room: &ShellRoom) -> Option<ShellResponse> {
        if command == "fleet" {
            Some(
                ShellResponse::simple(
                    "🚢 Fleet topology:\n\
                     └── plato-hub (this node)\n\
                         ├── puppeteer-agent [idle]\n\
                         ├── manus-agent [active]\n\
                         └── vision-agent [idle]",
                )
                .with_module_output("fleet:topology"),
            )
        } else {
            None
        }
    }

    fn room_for(&self, _exit: &str) -> Option<ShellRoom> {
        None
    }
}

/// A simple built-in module that provides rooms for navigation.
#[derive(Debug)]
struct PuppeteerModule;

impl PlatoModule for PuppeteerModule {
    fn name(&self) -> &str {
        "puppeteer"
    }

    fn handle_command(&self, command: &str, args: &str, _room: &ShellRoom) -> Option<ShellResponse> {
        if command == "puppeteer" || (command == "use" && args.contains("browser")) {
            Some(ShellResponse::simple("🎭 Puppeteer: Browser session active.").with_module_output("puppeteer:active"))
        } else {
            None
        }
    }

    fn room_for(&self, exit: &str) -> Option<ShellRoom> {
        if exit == "puppeteer" {
            let mut room = ShellRoom::new(
                "Puppeteer Chamber",
                "Rows of browser windows shimmer in the digital ether. Tabs flicker with activity.",
            );
            room.exits = vec!["hub".to_string()];
            room.objects = vec!["browser console".to_string(), "dom inspector".to_string()];
            room.module_source = "puppeteer".to_string();
            Some(room)
        } else {
            None
        }
    }
}

// ---------------------------------------------------------------------------
// Shell
// ---------------------------------------------------------------------------

/// The main shell — the agent's home environment.
pub struct Shell {
    config: ShellConfig,
    room: ShellRoom,
    history: Vec<String>,
    modules: Vec<Box<dyn PlatoModule>>,
    ticks: Vec<String>,
}

impl Shell {
    /// Create a new shell with the given configuration.
    pub fn new(config: ShellConfig) -> Self {
        let mut modules: Vec<Box<dyn PlatoModule>> = Vec::new();
        for name in &config.loaded_modules {
            if let Some(m) = Self::create_module(name) {
                modules.push(m);
            }
        }
        Self {
            config,
            room: ShellRoom::default_entry(),
            history: Vec::new(),
            modules,
            ticks: Vec::new(),
        }
    }

    fn create_module(name: &str) -> Option<Box<dyn PlatoModule>> {
        match name {
            "tick" => Some(Box::new(TickModule)),
            "fleet" => Some(Box::new(FleetModule)),
            "puppeteer" => Some(Box::new(PuppeteerModule)),
            _ => None,
        }
    }

    /// Process an agent command and return a response.
    pub fn process(&mut self, input: &str) -> ShellResponse {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return ShellResponse::simple("...");
        }

        // Record history
        self.push_history(trimmed.to_string());

        let (command, args) = match trimmed.split_once(' ') {
            Some((cmd, rest)) => (cmd, rest.trim()),
            None => (trimmed, ""),
        };

        match command {
            "look" | "examine" => self.cmd_look(args),
            "go" => self.cmd_go(args),
            "use" => self.cmd_use(args),
            "tick" => self.cmd_tick(args),
            "status" => self.cmd_status(),
            "fleet" => self.cmd_fleet(),
            "help" => self.cmd_help(),
            "modules" => self.cmd_modules(),
            _ => self.cmd_delegate(command, args),
        }
    }

    /// Get a reference to the current room.
    pub fn current_room(&self) -> &ShellRoom {
        &self.room
    }

    /// List available commands.
    pub fn available_commands(&self) -> Vec<String> {
        let mut cmds = vec![
            "look".to_string(),
            "go <exit>".to_string(),
            "use <object>".to_string(),
            "status".to_string(),
            "help".to_string(),
            "modules".to_string(),
        ];
        if self.has_module("tick") {
            cmds.push("tick <message>".to_string());
        }
        if self.has_module("fleet") {
            cmds.push("fleet".to_string());
        }
        cmds
    }

    /// Dynamically load a module.
    pub fn load_module(&mut self, name: &str) -> Result<(), String> {
        if self.has_module(name) {
            return Err(format!("module '{}' already loaded", name));
        }
        match Self::create_module(name) {
            Some(m) => {
                self.modules.push(m);
                if !self.config.loaded_modules.contains(&name.to_string()) {
                    self.config.loaded_modules.push(name.to_string());
                }
                Ok(())
            }
            None => Err(format!("unknown module '{}'", name)),
        }
    }

    /// Unload a module by name.
    pub fn unload_module(&mut self, name: &str) {
        self.modules.retain(|m| m.name() != name);
        self.config.loaded_modules.retain(|n| n != name);
    }

    /// Get command history.
    pub fn history(&self) -> &[String] {
        &self.history
    }

    // -- internal helpers --

    fn push_history(&mut self, entry: String) {
        if self.history.len() >= self.config.max_history {
            self.history.remove(0);
        }
        self.history.push(entry);
    }

    fn has_module(&self, name: &str) -> bool {
        self.modules.iter().any(|m| m.name() == name)
    }

    fn cmd_look(&self, args: &str) -> ShellResponse {
        if args.is_empty() {
            ShellResponse::simple(&self.room.to_string())
        } else {
            // Examine a specific object
            if self.room.objects.iter().any(|o| o == args) {
                ShellResponse::simple(&format!("You examine the {}. It seems ordinary... for now.", args))
            } else {
                ShellResponse::simple(&format!("You don't see '{}' here.", args))
            }
        }
    }

    fn cmd_go(&mut self, args: &str) -> ShellResponse {
        if args.is_empty() {
            return ShellResponse::simple("Go where? Specify an exit.");
        }

        // Special case: "hub" always returns to central hub
        if args == "hub" {
            let hub = ShellRoom::default_entry();
            self.room = hub.clone();
            return ShellResponse::with_room("You return to the Central Hub.", hub);
        }

        // Check built-in exits
        if self.room.exits.iter().any(|e| e == args) {
            // Try modules first
            for m in &self.modules {
                if let Some(room) = m.room_for(args) {
                    self.room = room.clone();
                    return ShellResponse::with_room(&format!("You head {}.", args), room);
                }
            }
            // Generic room
            let room = generic_room(args);
            self.room = room.clone();
            return ShellResponse::with_room(&format!("You head {}.", args), room);
        }

        ShellResponse::simple(&format!("You can't go '{}'. Try: {}", args, self.room.exits.join(", ")))
    }

    fn cmd_use(&self, args: &str) -> ShellResponse {
        if args.is_empty() {
            return ShellResponse::simple("Use what? Specify an object.");
        }
        // Delegate to modules
        for m in &self.modules {
            if let Some(resp) = m.handle_command("use", args, &self.room) {
                return resp;
            }
        }
        if self.room.objects.iter().any(|o| o == args) {
            ShellResponse::simple(&format!("You use the {}. Nothing happens... yet.", args))
        } else {
            ShellResponse::simple(&format!("There is no '{}' to use here.", args))
        }
    }

    fn cmd_tick(&mut self, args: &str) -> ShellResponse {
        for m in &self.modules {
            if let Some(resp) = m.handle_command("tick", args, &self.room) {
                self.ticks.push(args.to_string());
                return resp;
            }
        }
        ShellResponse::simple("Tick module not loaded. Use 'modules' to see what's available.")
    }

    fn cmd_fleet(&self) -> ShellResponse {
        for m in &self.modules {
            if let Some(resp) = m.handle_command("fleet", "", &self.room) {
                return resp;
            }
        }
        ShellResponse::simple("Fleet module not loaded. Use 'modules' to see what's available.")
    }

    fn cmd_status(&self) -> ShellResponse {
        let status = format!(
            "🟢 Plato Shell Status\n\
             Agent: {}\n\
             Model: {}\n\
             Room: {}\n\
             Modules: {}\n\
             History: {} commands\n\
             Ticks: {} pending",
            self.config.agent_name,
            self.config.model,
            self.room.title,
            if self.modules.is_empty() {
                "none".to_string()
            } else {
                self.modules.iter().map(|m| m.name().to_string()).collect::<Vec<_>>().join(", ")
            },
            self.history.len(),
            self.ticks.len(),
        );
        ShellResponse::simple(&status)
    }

    fn cmd_help(&self) -> ShellResponse {
        let cmds = self.available_commands().join("\n  ");
        ShellResponse::simple(&format!("Available commands:\n  {}", cmds))
    }

    fn cmd_modules(&self) -> ShellResponse {
        if self.modules.is_empty() {
            return ShellResponse::simple("No modules loaded. The shell is in basic MUD mode.");
        }
        let names: Vec<&str> = self.modules.iter().map(|m| m.name()).collect();
        ShellResponse::simple(&format!("Loaded modules: {}", names.join(", ")))
    }

    fn cmd_delegate(&self, command: &str, args: &str) -> ShellResponse {
        // Try to delegate to modules
        for m in &self.modules {
            if let Some(resp) = m.handle_command(command, args, &self.room) {
                return resp;
            }
        }
        ShellResponse::simple(&format!(
            "Unknown command '{}'. Type 'help' for available commands.",
            command
        ))
    }
}

fn generic_room(name: &str) -> ShellRoom {
    let mut room = ShellRoom::new(
        &format!("{} Chamber", name),
        &format!("A room belonging to the {} module. It's quiet here.", name),
    );
    room.exits = vec!["hub".to_string()];
    room.module_source = name.to_string();
    room
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn basic_config() -> ShellConfig {
        ShellConfig {
            loaded_modules: vec!["tick".to_string(), "fleet".to_string(), "puppeteer".to_string()],
            agent_name: "test-agent".to_string(),
            model: "test-model".to_string(),
            max_history: 100,
        }
    }

    fn empty_config() -> ShellConfig {
        ShellConfig {
            loaded_modules: vec![],
            agent_name: "bare-agent".to_string(),
            model: "bare".to_string(),
            max_history: 50,
        }
    }

    #[test]
    fn new_creates_shell_with_config() {
        let shell = Shell::new(basic_config());
        assert_eq!(shell.current_room().title, "Central Hub");
        assert_eq!(shell.history().len(), 0);
    }

    #[test]
    fn look_describes_current_room() {
        let mut shell = Shell::new(basic_config());
        let resp = shell.process("look");
        assert!(resp.text.contains("Central Hub"));
        assert!(!resp.room_changed);
    }

    #[test]
    fn go_navigates_to_new_room() {
        let mut shell = Shell::new(basic_config());
        let resp = shell.process("go puppeteer");
        assert!(resp.room_changed);
        assert!(resp.new_room.is_some());
        assert_eq!(resp.new_room.unwrap().title, "Puppeteer Chamber");
        assert_eq!(shell.current_room().title, "Puppeteer Chamber");
    }

    #[test]
    fn unknown_command_returns_help_hint() {
        let mut shell = Shell::new(basic_config());
        let resp = shell.process("foobar");
        assert!(resp.text.contains("Unknown command"));
        assert!(resp.text.contains("help"));
    }

    #[test]
    fn help_lists_available_commands() {
        let mut shell = Shell::new(basic_config());
        let resp = shell.process("help");
        assert!(resp.text.contains("look"));
        assert!(resp.text.contains("go"));
        assert!(resp.text.contains("tick"));
        assert!(resp.text.contains("fleet"));
    }

    #[test]
    fn status_returns_system_info() {
        let mut shell = Shell::new(basic_config());
        let resp = shell.process("status");
        assert!(resp.text.contains("test-agent"));
        assert!(resp.text.contains("test-model"));
        assert!(resp.text.contains("Central Hub"));
    }

    #[test]
    fn load_module_adds_module() {
        let mut shell = Shell::new(empty_config());
        assert!(shell.load_module("tick").is_ok());
        let resp = shell.process("tick hello");
        assert!(resp.text.contains("Tick sent"));
    }

    #[test]
    fn unload_module_removes_module() {
        let mut shell = Shell::new(basic_config());
        assert!(shell.modules.iter().any(|m| m.name() == "tick"));
        shell.unload_module("tick");
        assert!(!shell.modules.iter().any(|m| m.name() == "tick"));
    }

    #[test]
    fn history_tracks_commands() {
        let mut shell = Shell::new(basic_config());
        shell.process("look");
        shell.process("status");
        shell.process("help");
        assert_eq!(shell.history().len(), 3);
        assert_eq!(shell.history()[0], "look");
        assert_eq!(shell.history()[2], "help");
    }

    #[test]
    fn module_specific_commands_delegated() {
        let mut shell = Shell::new(basic_config());
        let resp = shell.process("puppeteer");
        assert!(resp.module_output.is_some());
        assert!(resp.module_output.unwrap().contains("puppeteer"));
    }

    #[test]
    fn tick_command_creates_a_tick() {
        let mut shell = Shell::new(basic_config());
        let resp = shell.process("tick hello world");
        assert!(resp.text.contains("Tick sent"));
        assert!(resp.text.contains("hello world"));
        assert!(resp.events.iter().any(|e| e.contains("tick:")));
    }

    #[test]
    fn fleet_command_shows_fleet_status() {
        let mut shell = Shell::new(basic_config());
        let resp = shell.process("fleet");
        assert!(resp.text.contains("Fleet topology"));
        assert!(resp.module_output.is_some());
    }

    #[test]
    fn shell_works_with_zero_modules() {
        let mut shell = Shell::new(empty_config());
        let resp = shell.process("look");
        assert!(resp.text.contains("Central Hub"));
        let resp = shell.process("help");
        assert!(resp.text.contains("look"));
        // tick should fail gracefully
        let resp = shell.process("tick hi");
        assert!(resp.text.contains("not loaded"));
        // fleet should fail gracefully
        let resp = shell.process("fleet");
        assert!(resp.text.contains("not loaded"));
        // but basic MUD navigation still works
        let resp = shell.process("go puppeteer");
        assert!(resp.room_changed);
    }

    #[test]
    fn examine_specific_object() {
        let mut shell = Shell::new(basic_config());
        let resp = shell.process("examine status terminal");
        assert!(resp.text.contains("status terminal"));
    }

    #[test]
    fn go_back_to_hub() {
        let mut shell = Shell::new(basic_config());
        shell.process("go puppeteer");
        assert_eq!(shell.current_room().title, "Puppeteer Chamber");
        let _resp = shell.process("go hub");
        assert_eq!(shell.current_room().title, "Central Hub");
    }

    #[test]
    fn go_invalid_exit() {
        let mut shell = Shell::new(basic_config());
        let resp = shell.process("go nowhere");
        assert!(resp.text.contains("can't go"));
    }

    #[test]
    fn load_duplicate_module_fails() {
        let mut shell = Shell::new(basic_config());
        assert!(shell.load_module("tick").is_err());
    }

    #[test]
    fn load_unknown_module_fails() {
        let mut shell = Shell::new(empty_config());
        assert!(shell.load_module("nonexistent").is_err());
    }

    #[test]
    fn use_existing_object() {
        let mut shell = Shell::new(basic_config());
        let resp = shell.process("use status terminal");
        assert!(resp.text.contains("status terminal"));
    }

    #[test]
    fn use_nonexistent_object() {
        let mut shell = Shell::new(basic_config());
        let resp = shell.process("use magic wand");
        assert!(resp.text.contains("no 'magic wand'"));
    }

    #[test]
    fn modules_command() {
        let mut shell = Shell::new(basic_config());
        let resp = shell.process("modules");
        assert!(resp.text.contains("tick"));
        assert!(resp.text.contains("fleet"));
    }
}
