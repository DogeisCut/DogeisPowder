use minifb::{Key, MouseButton, Window};
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum Action {
    UseTool,
    EnlargeToolShape,
    ShrinkToolShape,
    SwitchToolShape,
}

#[derive(Debug, Clone, Copy)]
pub enum RawInput {
    Keyboard(Key),
    Mouse(MouseButton),
    ScrollUp,
    ScrollDown,
}

pub struct InputMap {
    mappings: HashMap<Action, Vec<RawInput>>,
}

// the following makes me very sad!

impl PartialEq for RawInput {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RawInput::Keyboard(k1), RawInput::Keyboard(k2)) => k1 == k2,
            (RawInput::Mouse(m1), RawInput::Mouse(m2)) => (*m1 as u8) == (*m2 as u8),
            (RawInput::ScrollUp, RawInput::ScrollUp) => true,
            (RawInput::ScrollDown, RawInput::ScrollDown) => true,
            _ => false,
        }
    }
}

impl Eq for RawInput {}

impl Hash for RawInput {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            RawInput::Keyboard(key) => {
                state.write_u8(0);
                key.hash(state);
            }
            RawInput::Mouse(button) => {
                state.write_u8(1);
                (*button as u8).hash(state);
            }
            RawInput::ScrollUp => {
                state.write_u8(2);
            }
            RawInput::ScrollDown => {
                state.write_u8(3);
            }
        }
    }
}

// okay enough of that junk

impl InputMap {
    pub fn new() -> Self {
        Self {
            mappings: HashMap::new(),
        }
    }

    pub fn add_action_event(&mut self, action: Action, input: RawInput) {
        self.mappings.entry(action).or_default().push(input);
    }
}

#[derive(Default)]
pub struct InputState {
    pub mouse_pos: (f32, f32),
    pub scroll_wheel: (f32, f32),
    active_actions: HashSet<Action>,
}
impl InputState {
    pub fn is_action_pressed(&self, action: Action) -> bool {
        self.active_actions.contains(&action)
    }

    pub fn is_action_just_pressed(&self, action: Action) -> bool {
        // TODO: implement
        self.active_actions.contains(&action)
    }

    pub fn is_action_just_released(&self, action: Action) -> bool {
        // TODO: implement
        self.active_actions.contains(&action)
    }

    pub fn update(&mut self, window: &Window, input_map: &InputMap) {
        // Update mouse position
        if let Some((mx, my)) = window.get_mouse_pos(minifb::MouseMode::Clamp) {
            self.mouse_pos = (mx, my);
        }

        self.scroll_wheel = (0.0, 0.0);
        if let Some((mx, my)) = window.get_scroll_wheel() {
            self.scroll_wheel = (mx, my);
        }

        self.active_actions.clear();

        for (action, physical_inputs) in &input_map.mappings {
            for input in physical_inputs {
                let is_pressed = match input {
                    RawInput::Keyboard(key) => window.is_key_down(*key),
                    RawInput::Mouse(button) => window.get_mouse_down(*button),
                    RawInput::ScrollUp => self.scroll_wheel.1 > 0.0,
                    RawInput::ScrollDown => self.scroll_wheel.1 < 0.0,
                };

                if is_pressed {
                    self.active_actions.insert(*action);
                    break;
                }
            }
        }
    }
}
