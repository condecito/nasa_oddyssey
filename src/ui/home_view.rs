use druid::{Env, EventCtx, Widget};
use druid::widget::Label;
use crate::data::AppState;
use crate::data::home::HomeView;
use crate::data::navigation::Route;


impl HomeView {
    pub fn new() -> impl Widget<AppState> {
        Self::build_home_widget()
    }

    pub fn btn_click_home(ctx: &mut EventCtx, data: &mut AppState, _env: &Env) {
        data.navigate_to(Route::Home);
        ctx.request_update();
    }

    fn build_home_widget()->impl Widget<AppState>{
        Label::new("Home View")
    }

}
