//! template-app: state -> UI model, after every action.

use crate::state::State;
use crate::ui_model::UiModel;
use everyware_ui::slot;
use harry::presentation::Presenter;

pub struct UiModelPresenter;

impl Presenter<State, UiModel> for UiModelPresenter {
    fn present(model: &mut UiModel, state: &State) {
        model.time = state.clock.time.clone();
        model.count = state.count.to_string();
        model.tap = slot("#template/tap", &state.tap);
    }
}
