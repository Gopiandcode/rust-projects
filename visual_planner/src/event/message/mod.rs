pub mod renderer;
pub mod gui;
pub mod manager;

use state::DialogInputState;
use types::*;
use manager::draw_view::Drawable;
use std::sync::Arc;

/// A thread-safe wrapper for all messages sent 
#[derive(Debug,Clone)]
pub enum GeneralMessage {
    RendererScreenResize(ScreenUnit, ScreenUnit),
    RendererScroll(ScreenUnit, ScreenUnit, ScrollDirection, f64),
    RendererClick(ScreenUnit, ScreenUnit),
    RendererMotion(ScreenUnit, ScreenUnit),
    Redraw(GuiWidgetID),
    SetDialogInputState(DialogInputState),
    SetCursor(GuiWidgetID, &'static str),
    WindowMove(ScreenUnit, ScreenUnit),
    ConstructResult(Arc<Drawable>)
}
