use druid::Data;

#[derive(Copy, Clone, Debug, Data, PartialEq, Eq, Hash)]
pub enum Route {
    Home,
    Launcher,
    Apod,
    Neows,
    Rover
}