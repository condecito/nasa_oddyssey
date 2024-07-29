use druid::Color;
use lazy_static::lazy_static;

pub mod icons;

lazy_static! {
    pub static ref BACKGROUND_COLOR: Color = Color::rgb(26.0 / 255.0, 32.0 / 255.0, 41.0 / 255.0);
    pub static ref BORDER_COLOR: Color = Color::rgb(79.0 / 255.0, 156.0 / 255.0, 164.0 / 255.0);

}