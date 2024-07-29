
pub mod navigation;
pub mod home;
pub mod apod;
pub mod launcher;
pub mod rover;
pub mod newos;

use druid::{Data, Lens};
use druid::im::Vector;
use crate::data::apod::ApodView;
use crate::data::launcher::LauncherView;
use crate::data::navigation::Route;


#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub api_key: String,
    pub route: Route,
    pub launches: Vector<LauncherView>,
    pub apod:ApodView
}

impl AppState {
    pub fn navigate_to(&mut self, route: Route) {
        self.route = route;
    }
}

