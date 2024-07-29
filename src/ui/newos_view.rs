use druid::{Env, EventCtx, Widget};
use druid::widget::Label;
use crate::data::AppState;
use crate::data::launcher::LauncherView;
use crate::data::navigation::Route;
use crate::data::newos::NeowsView;

impl NeowsView {
    pub fn new() -> impl Widget<AppState> {
        Self::build_newos_widget()
    }
    pub fn btn_click_newos(ctx: &mut EventCtx, data: &mut AppState, _env: &Env) {
        data.navigate_to(Route::Neows);
        ctx.request_update();
    }

    fn build_newos_widget() -> impl Widget<AppState> {
        Label::new("newos view")
    }
}