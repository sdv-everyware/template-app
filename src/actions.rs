//! template-app: the clock's tick and a gesture on a node. A tap on the button counts up.

use crate::clock::Clock;
use crate::state::State;
use everyware_ui::button::ButtonEvent;
use everyware_ui::Gesture;
use harry::side_effects::SideEffects;
use harry::state::StateAction;

#[derive(Clone, Debug, PartialEq)]
pub enum TemplateAction {
    /// Once a second from the ticker thread (main.rs).
    Tick(Clock),
    /// A gesture on a node (ui_event_listener.rs).
    Gesture(String, Gesture),
}

impl StateAction<State, ()> for TemplateAction {
    fn apply(&self, state: &mut State, _side_effects: &mut Vec<SideEffects<()>>) {
        match self {
            TemplateAction::Tick(now) => state.clock = now.clone(),
            TemplateAction::Gesture(node, gesture) => on_gesture(state, node, *gesture),
        }
    }
}

/// The node's name decides which component gets the gesture; the component answers with an
/// event when something completed.
fn on_gesture(state: &mut State, node: &str, gesture: Gesture) {
    if node == "#template/tap" && state.tap.on(gesture) == Some(ButtonEvent::Clicked) {
        state.count += 1;
        log::info!("template: tapped {} times", state.count);
    }
}
