//! template-app: everything the screen knows: the clock, the number of taps and the button.

use crate::clock::Clock;
use everyware_ui::button::{Button, ButtonKind};

pub struct State {
    pub clock: Clock,
    /// How often the button was tapped.
    pub count: u32,
    /// The library's button (#button in a slot): it owns its pressed state.
    pub tap: Button,
}

impl State {
    pub fn new() -> Self {
        State { clock: Clock::default(), count: 0, tap: Button::new(ButtonKind::Primary, "Tap me") }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
