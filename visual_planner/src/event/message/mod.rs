pub mod renderer;
pub mod gui;
pub mod manager;

use state::DialogInputState;
use manager::draw_view::Drawable;
use manager::components::boxes::BoxConstructor;
use types::*;


use std::sync::Arc;

/// A thread-safe wrapper for all messages sent 
#[derive(Debug,Clone)]
pub enum GeneralMessage {
    RendererScreenResize(ScreenUnit, ScreenUnit),
    RendererScroll(ScreenUnit, ScreenUnit, ScrollDirection, f64),
    RendererClick(ScreenUnit, ScreenUnit),
    RendererMotion(ScreenUnit, ScreenUnit),
    DialogRedraw(WorldBoundingBox),
    SetDialogInputState(DialogInputState),
    SetCursor(GuiWidgetID, &'static str),
    WindowMove(ScreenUnit, ScreenUnit),
    BoxConstructRequest(BoxConstructor),
    ConstructResult(Arc<Drawable>),
    DialogTimer(CurrentTime,DeltaTime)
}
