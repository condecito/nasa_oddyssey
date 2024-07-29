use druid::{Env, EventCtx, RenderContext, Widget, WidgetExt};


use druid::widget::{Flex, Label, List, Painter, Scroll};
use crate::assets::BORDER_COLOR;
use crate::data::{AppState};
use crate::data::app_state_derived_lenses::launches;
use crate::data::launcher::LauncherView;
use crate::data::navigation::Route;

impl LauncherView {
    pub fn new() -> impl Widget<AppState> {
       Label::new("asdsa")
    }
    pub fn btn_click_launcher(ctx: &mut EventCtx, data: &mut AppState, _env: &Env) {
        data.navigate_to(Route::Launcher);
        ctx.request_update();
    }

/*    fn build_launcher_widget() -> impl Widget<AppState> {
        List::new(|| {
           Flex::row()
               .with_child(Label::new("qwe").lens(LauncherView::title))
               .with_default_spacer()
               .with_child(Label::new("asdasd").lens(LauncherView::date))
                         })
            .lens(AppState::launches)
    }*/
}