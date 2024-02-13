mod zoom_controls;
mod zoom_controls_imp;
use glib::subclass::SignalInvocationHint;
pub use zoom_controls::*;

mod style_switcher;
mod style_switcher_imp;
pub use style_switcher::*;

mod panel;
mod panel_imp;
pub use panel::*;

mod tile_header;
pub use tile_header::*;

mod paned;
mod paned_imp;
pub use paned::*;

mod panel_grid;
mod panel_grid_imp;
pub use panel_grid::*;

pub fn signal_accumulator_propagation(_hint: &SignalInvocationHint, return_accu: &mut glib::Value, handler_return: &glib::Value) -> bool {
    let signal_propagate = glib::Propagation::from(handler_return.get::<bool>().unwrap_or(true));

    *return_accu = handler_return.clone();
    signal_propagate.into()
}
