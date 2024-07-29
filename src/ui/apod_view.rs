use druid::{Color, Env, EventCtx, UnitPoint, Widget, WidgetExt, WidgetId};

use druid::widget::{Flex, Label, LineBreaking, Scroll};
use crate::assets::BORDER_COLOR;
use crate::data::apod::ApodView;
use crate::data::AppState;

use crate::data::navigation::Route;
const TITLE_ID: WidgetId = WidgetId::reserved(1);
const BODY_ID: WidgetId = WidgetId::reserved(2);
const MAIN_CONTAINER: WidgetId = WidgetId::reserved(3);
impl ApodView {
    pub fn new() -> impl Widget<AppState> {
     /*   let title = Flex::column()

            .with_child(Label::new(" Astronomy Picture of the Day (APOD)"))
            .border(*BORDER_COLOR, 1.)
            .with_id(TITLE_ID)
            ;*/

        let body = Flex::column()
            .with_default_spacer()
            .with_child(Label::new(" Astronomy Picture of the Day (APOD)"))
            .with_default_spacer()
            .with_child(Label::new(|data: &AppState, _env: &_| data.apod.title.clone()))
            .with_default_spacer()
            .with_child(Label::new(|data: &AppState, _env: &_| data.apod.date.clone()))
            .with_default_spacer()
            .with_child(
                Scroll::new(
                    Label::new(|data: &AppState, _env: &_| data.apod.explanation.clone())
                        .with_line_break_mode(LineBreaking::WordWrap)
                        .align_horizontal(UnitPoint::LEFT)
                        .padding(10.)
                ).vertical()
            ).with_child(Label::new("Aca va una fotp"))
            .border(Color::WHITE, 1.)
            .with_id(BODY_ID)
            .padding(5.)
            ;

        return body;
/*
       Flex::column().with_default_spacer()
            .with_flex_child(title, 1.)
            .with_default_spacer()
            .with_child(body)
            .border(Color::RED, 2.)
           .with_id(MAIN_CONTAINER)*/
    }
    pub fn btn_click_apod(ctx: &mut EventCtx, data: &mut AppState, _env: &Env) {
        data.navigate_to(Route::Apod);
        ctx.request_update();
    }
}