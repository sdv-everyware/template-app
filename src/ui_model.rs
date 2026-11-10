//! template-app: the UI model, one field per layer of #template-root the app writes to or taps
//! (design: design/template-patch.js). The names are the contract with the design: rename both
//! or neither.

use everyware_ui::model::prelude::*;

everyware_ui::ui_model! {
    time: String = "#template/time";
    count: String = "#template/count";
    tap: Slot = "#template/tap" ["SlotCustomizer"];
}
