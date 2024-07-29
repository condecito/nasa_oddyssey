use std::vec;
use druid::{AppLauncher, WindowDesc};
use druid::im::{Vector};

use crate::data::AppState;
use crate::data::launcher::LauncherView;

use crate::data::navigation::Route;


mod ui;
mod data;
mod core;
mod assets;

use crate::core::client;

#[tokio::main]
async fn main() {
    let main_window = WindowDesc::new(ui::ui_builder())
        .window_size(druid::Size::new(800., 600.0))
        .title("NASA Oddyssey")
        .transparent(true)
        .show_titlebar(true);

    let t = (client::apod().await.unwrap());
    let data = AppState {
        api_key: String::from("API-KEY645465654"),
        route: Route::Home,
        launches: Vector::new(),
        apod:t
    };

    AppLauncher::with_window(main_window)
        .log_to_console()

        .launch(data)
        .expect("Expect the app running");
}


/*use reqwest::Error;
use crate::core::client::apod;

mod core;

#[tokio::main]
async fn main() -> Result<(), Error> {
    apod().await?;
    Ok(())
}*/

/*
from(vec![
            LauncherView {
                copyright: String::from("sadasd"),
                date: "1".to_string(),
                explanation: "2".to_string(),
                media_type: "3".to_string(),
                title: "4".to_string(),
                hdurl: "5".to_string(),
                url: "6".to_string(),
            }]
*/