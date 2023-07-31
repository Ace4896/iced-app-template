use iced::{Application, Settings};

mod app;

fn main() -> iced::Result {
    app::App::run(Settings::default())
}
