pub mod launchers_view;
pub mod home_view;
mod apod_view;
mod rover_view;
mod newos_view;

use std::io::Empty;
use std::str::FromStr;

use druid::{Env, Widget, WidgetExt};
use druid::widget::{CrossAxisAlignment, Flex, Label, LabelText, List, Split, Svg, SvgData, ViewSwitcher};

use crate::assets::{BACKGROUND_COLOR, BORDER_COLOR};
use crate::assets::icons::{ALIEN, METEOR_2, START_SHIP_4, TELESCOPE, URL_NAVIGATOR};
use crate::data::apod::{ApodView};
use crate::data::{AppState, launcher};
use crate::data::home::HomeView;
use crate::data::launcher::LauncherView;
use crate::data::navigation::Route;
use crate::data::navigation::Route::Home;
use crate::data::newos::NeowsView;
use crate::data::rover::RoverView;


pub fn ui_builder() -> impl Widget<AppState> {
    let split = Split::columns(build_side_widget(), build_main_widget())
        .split_point(0.009)
        .bar_size(1.0)
        .min_size(40., 40.)
        .min_bar_area(0.5)
        .solid_bar(true)
        .background(*BACKGROUND_COLOR);
    return split;
}


fn build_main_widget() -> impl Widget<AppState> {
    let main = Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(build_right_widget())
        .expand_width()
        .border(*BORDER_COLOR,1.);
    return main;
}

fn build_right_widget() -> impl Widget<AppState> {
    ViewSwitcher::new(
        |data: &AppState, _env| data.route,
        |route, _data, _env| match route {
            Home => Box::new(HomeView::new()),
            Route::Launcher => Box::new(LauncherView::new()),
            Route::Apod => Box::new(ApodView::new()),
            Route::Neows => Box::new(NeowsView::new()),
            Route::Rover => Box::new(RoverView::new()),
        },
    )
}


fn build_side_widget() -> impl Widget<AppState> {
    let svg_home = SvgData::from_str(URL_NAVIGATOR).unwrap();
    let svg_home = Svg::new(svg_home.clone())
        .fix_size(32., 32.)
        .on_click(HomeView::btn_click_home)

        ;
    let lef_panel_top = Flex::column()
        .with_flex_spacer(0.10)
        .with_flex_child(svg_home, 1.0)
        .with_default_spacer();
    let svg_launch = SvgData::from_str(START_SHIP_4).unwrap();
    let svg_launch = Svg::new(svg_launch.clone()).fix_size(32., 32.)
        .on_click(LauncherView::btn_click_launcher)
        ;
    let svg_apod = SvgData::from_str(TELESCOPE).unwrap();
    let svg_apod = Svg::new(svg_apod.clone()).fix_size(32., 32.)
        .on_click(ApodView::btn_click_apod);
    let svg_neows = SvgData::from_str(METEOR_2).unwrap();
    let svg_neows = Svg::new(svg_neows.clone()).fix_size(32., 32.)
        .on_click(NeowsView::btn_click_newos);
    ;

    let svg_rover = SvgData::from_str(ALIEN).unwrap();
    let svg_rover = Svg::new(svg_rover.clone()).fix_size(32., 32.)
        .on_click(RoverView::btn_click_roverr);
    ;
    let let_panel_mid = Flex::column()
        .with_flex_child(svg_launch, 1.)//calendar
        .with_flex_spacer(0.40)
        .with_flex_child(svg_apod, 1.0)//picture of the day
        .with_flex_spacer(0.50)
        .with_flex_child(svg_neows, 1.0)//object near to earth
        .with_flex_spacer(0.50)
        .with_flex_child(svg_rover, 1.0)//mars robber photos
        .center();

    Split::rows(lef_panel_top, let_panel_mid)
        .split_point(0.360)
        .bar_size(1.0)
        .min_size(40.0, 120.0)
        .min_bar_area(0.3)
        .solid_bar(false)
}

/*fn build_next_event_widget() -> impl Widget<AppState> {
    List::new(|| {
        Flex::column()
            .with_child(Label::new(|data: &ApodView, _env: &_| data.title.clone())
                .padding(10.0)
            )
            .with_default_spacer()
            .with_child(Label::new(|data: &ApodView, _env: &_| data.date.clone())
                .padding(10.0)
            )
            .with_default_spacer()
            .with_child(Label::new(|data: &ApodView, _env: &_| data.explanation.clone())
                .padding(10.0)
            )
    })
        .lens(AppState::launches)
}*/

/*fn build_apod_widget() -> impl Widget<AppState> {
    List::new(|| {
        Flex::column()
            .with_child(Label::new(|data: &LauncherView, _env: &_| data.title.clone())
                .padding(10.0)
            )
            .with_default_spacer()
            .with_child(Label::new(|data: &LauncherView, _env: &_| data.date.clone())
                .padding(10.0)
            )
            .with_default_spacer()
            .with_child(Label::new(|data: &LauncherView, _env: &_| data.explanation.clone())
                .padding(10.0)
            )
    })
        .lens(AppState::launches)
}*/

fn build_object_near_widget() -> impl Widget<AppState> {
    Flex::column()
        .with_child(Label::new(LabelText::from("object_near")))
}

fn build_robert_photos_widget() -> impl Widget<AppState> {
    Flex::column()
        .with_child(Label::new(LabelText::from("robert_photos")))
}