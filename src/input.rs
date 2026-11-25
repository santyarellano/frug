use std::collections::HashSet;

use sdl3::event::Event;
use sdl3::keyboard::Keycode;

/// Keyboard state tracked across frames.
///
/// * `down` - the key is currently held.
/// * `pressed` - the key went down since the last game update (key-repeat is ignored).
/// * `released` - the key went up since the last game update.
///
/// When used through [`crate::run`] the pressed/released sets are cleared after every
/// fixed update, so each press is seen by exactly one `update` call.
#[derive(Default, Debug, Clone)]
pub struct Input {
    down: HashSet<Keycode>,
    pressed: HashSet<Keycode>,
    released: HashSet<Keycode>,
}

impl Input {
    pub fn new() -> Self {
        Self::default()
    }

    /// Feed an SDL event into the input state.
    pub fn handle_event(&mut self, event: &Event) {
        match event {
            Event::KeyDown {
                keycode: Some(k),
                repeat,
                ..
            } => self.key_down(*k, *repeat),
            Event::KeyUp {
                keycode: Some(k), ..
            } => self.key_up(*k),
            _ => {}
        }
    }

    /// Register a key-down. Repeats are ignored for `pressed`.
    pub fn key_down(&mut self, key: Keycode, repeat: bool) {
        if repeat {
            return;
        }
        if self.down.insert(key) {
            self.pressed.insert(key);
        }
    }

    /// Register a key-up.
    pub fn key_up(&mut self, key: Keycode) {
        if self.down.remove(&key) {
            self.released.insert(key);
        }
    }

    /// Clear the one-shot pressed/released sets. Call after each update.
    pub fn clear_transitions(&mut self) {
        self.pressed.clear();
        self.released.clear();
    }

    /// Forget everything (e.g. when the window loses focus).
    pub fn reset(&mut self) {
        self.down.clear();
        self.clear_transitions();
    }

    pub fn down(&self, key: Keycode) -> bool {
        self.down.contains(&key)
    }

    pub fn pressed(&self, key: Keycode) -> bool {
        self.pressed.contains(&key)
    }

    pub fn released(&self, key: Keycode) -> bool {
        self.released.contains(&key)
    }

    /// `1` if `positive` is held, `-1` if `negative` is held, `0` if both or neither.
    pub fn axis(&self, negative: Keycode, positive: Keycode) -> i32 {
        self.down(positive) as i32 - self.down(negative) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn press_is_one_shot_and_hold_persists() {
        let mut i = Input::new();
        i.key_down(Keycode::Space, false);
        assert!(i.pressed(Keycode::Space));
        assert!(i.down(Keycode::Space));
        i.clear_transitions();
        assert!(!i.pressed(Keycode::Space));
        assert!(i.down(Keycode::Space));
    }

    #[test]
    fn repeat_does_not_retrigger_pressed() {
        let mut i = Input::new();
        i.key_down(Keycode::A, false);
        i.clear_transitions();
        i.key_down(Keycode::A, true);
        assert!(!i.pressed(Keycode::A));
        // A non-repeat down while already held is also not a new press.
        i.key_down(Keycode::A, false);
        assert!(!i.pressed(Keycode::A));
    }

    #[test]
    fn release_tracks_up() {
        let mut i = Input::new();
        i.key_down(Keycode::Return, false);
        i.key_up(Keycode::Return);
        assert!(i.pressed(Keycode::Return));
        assert!(i.released(Keycode::Return));
        assert!(!i.down(Keycode::Return));
        // Release of an unheld key is ignored.
        i.clear_transitions();
        i.key_up(Keycode::Return);
        assert!(!i.released(Keycode::Return));
    }

    #[test]
    fn axis() {
        let mut i = Input::new();
        assert_eq!(i.axis(Keycode::Left, Keycode::Right), 0);
        i.key_down(Keycode::Right, false);
        assert_eq!(i.axis(Keycode::Left, Keycode::Right), 1);
        i.key_down(Keycode::Left, false);
        assert_eq!(i.axis(Keycode::Left, Keycode::Right), 0);
        i.key_up(Keycode::Right);
        assert_eq!(i.axis(Keycode::Left, Keycode::Right), -1);
    }
}
