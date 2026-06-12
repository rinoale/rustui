use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommonIntent {
    EnterCommandMode,
    Cancel,
    Help,
    NextPane,
    PreviousPane,
    Activate,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Search,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Char(char),
    Esc,
    F(u8),
    Enter,
    Tab,
    BackTab,
    Up,
    Down,
    Left,
    Right,
    PageUp,
    PageDown,
    Home,
    End,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyPattern {
    pub key: Key,
    pub modifiers: KeyModifiers,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyBinding<I> {
    pub pattern: KeyPattern,
    pub intent: I,
    pub label: &'static str,
    pub description: &'static str,
}

#[derive(Debug, Clone)]
pub struct Keymap<I> {
    bindings: Vec<KeyBinding<I>>,
}

impl Default for Keymap<CommonIntent> {
    fn default() -> Self {
        common_keymap()
    }
}

impl<I> Keymap<I> {
    pub fn new(bindings: impl Into<Vec<KeyBinding<I>>>) -> Self {
        Self {
            bindings: bindings.into(),
        }
    }

    pub fn with_binding(mut self, binding: KeyBinding<I>) -> Self {
        self.bindings
            .retain(|existing| existing.pattern != binding.pattern);
        self.bindings.push(binding);
        self
    }

    pub fn without_key(mut self, pattern: KeyPattern) -> Self {
        self.bindings.retain(|existing| existing.pattern != pattern);
        self
    }

    pub fn bindings(&self) -> &[KeyBinding<I>] {
        &self.bindings
    }
}

impl<I: Copy> Keymap<I> {
    pub fn intent_for(&self, key: KeyEvent) -> Option<I> {
        self.bindings
            .iter()
            .find(|binding| binding.pattern.matches(key))
            .map(|binding| binding.intent)
    }
}

pub fn common_keymap() -> Keymap<CommonIntent> {
    let none = KeyModifiers::NONE;
    Keymap::new(vec![
        binding(
            Key::Char(':'),
            none,
            CommonIntent::EnterCommandMode,
            ":",
            "command mode",
        ),
        binding(Key::Esc, none, CommonIntent::Cancel, "Esc", "cancel"),
        binding(Key::Char('?'), none, CommonIntent::Help, "?", "help"),
        binding(Key::Tab, none, CommonIntent::NextPane, "Tab", "next pane"),
        binding(
            Key::BackTab,
            none,
            CommonIntent::PreviousPane,
            "Shift-Tab",
            "previous pane",
        ),
        binding(
            Key::Enter,
            none,
            CommonIntent::Activate,
            "Enter",
            "activate",
        ),
        binding(Key::Up, none, CommonIntent::MoveUp, "Up", "move up"),
        binding(Key::Down, none, CommonIntent::MoveDown, "Down", "move down"),
        binding(Key::Left, none, CommonIntent::MoveLeft, "Left", "move left"),
        binding(
            Key::Right,
            none,
            CommonIntent::MoveRight,
            "Right",
            "move right",
        ),
        binding(Key::Char('/'), none, CommonIntent::Search, "/", "search"),
    ])
}

pub fn binding<I>(
    key: Key,
    modifiers: KeyModifiers,
    intent: I,
    label: &'static str,
    description: &'static str,
) -> KeyBinding<I> {
    KeyBinding {
        pattern: KeyPattern { key, modifiers },
        intent,
        label,
        description,
    }
}

impl KeyPattern {
    pub fn matches(self, event: KeyEvent) -> bool {
        key_matches(self.key, event.code) && normalized_modifiers(event.modifiers) == self.modifiers
    }
}

pub fn text_input_modifiers(mut modifiers: KeyModifiers) -> bool {
    modifiers.remove(KeyModifiers::SHIFT);
    modifiers.is_empty()
}

fn key_matches(expected: Key, actual: KeyCode) -> bool {
    match (expected, actual) {
        (Key::Char(expected), KeyCode::Char(actual)) => expected == actual,
        (Key::Esc, KeyCode::Esc) => true,
        (Key::F(expected), KeyCode::F(actual)) => expected == actual,
        (Key::Enter, KeyCode::Enter) => true,
        (Key::Tab, KeyCode::Tab) => true,
        (Key::BackTab, KeyCode::BackTab) => true,
        (Key::Up, KeyCode::Up) => true,
        (Key::Down, KeyCode::Down) => true,
        (Key::Left, KeyCode::Left) => true,
        (Key::Right, KeyCode::Right) => true,
        (Key::PageUp, KeyCode::PageUp) => true,
        (Key::PageDown, KeyCode::PageDown) => true,
        (Key::Home, KeyCode::Home) => true,
        (Key::End, KeyCode::End) => true,
        _ => false,
    }
}

fn normalized_modifiers(mut modifiers: KeyModifiers) -> KeyModifiers {
    modifiers.remove(KeyModifiers::SHIFT);
    modifiers
}

#[cfg(test)]
mod tests {
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    use super::{CommonIntent, Key, Keymap, binding, common_keymap, text_input_modifiers};

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum LocalIntent {
        Open,
    }

    #[test]
    fn common_keymap_has_no_quit_key() {
        let keymap = common_keymap();

        assert_eq!(
            keymap.intent_for(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE)),
            None
        );
        assert_eq!(
            keymap.intent_for(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL)),
            None
        );
    }

    #[test]
    fn colon_enters_command_mode() {
        let keymap = common_keymap();

        assert_eq!(
            keymap.intent_for(KeyEvent::new(KeyCode::Char(':'), KeyModifiers::NONE)),
            Some(CommonIntent::EnterCommandMode)
        );
    }

    #[test]
    fn apps_can_use_local_intents() {
        let keymap = Keymap::new(vec![binding(
            Key::F(5),
            KeyModifiers::NONE,
            LocalIntent::Open,
            "F5",
            "open",
        )]);

        assert_eq!(
            keymap.intent_for(KeyEvent::new(KeyCode::F(5), KeyModifiers::NONE)),
            Some(LocalIntent::Open)
        );
    }

    #[test]
    fn text_input_allows_shifted_characters() {
        assert!(text_input_modifiers(KeyModifiers::SHIFT));
        assert!(text_input_modifiers(KeyModifiers::NONE));
        assert!(!text_input_modifiers(KeyModifiers::CONTROL));
    }
}
