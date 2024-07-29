use druid::{Env, EventCtx, Widget};
use druid::widget::Label;
use crate::data::AppState;

use crate::data::navigation::Route;
use crate::data::rover::RoverView;

impl RoverView{
    pub fn new() -> impl Widget<AppState> {
        Self::build_rover_widget()
    }
    pub fn btn_click_roverr(ctx: &mut EventCtx, data: &mut AppState, _env: &Env) {
        data.navigate_to(Route::Rover);
        ctx.request_update();
    }

    fn build_rover_widget() -> impl Widget<AppState> {
        Label::new("rover view")
    }
}