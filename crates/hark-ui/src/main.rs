#![windows_subsystem = "windows"]

mod app;
mod ui;

use app::Hark;

fn main() -> iced::Result {
    iced::application(Hark::new, Hark::update, Hark::view)
        .subscription(Hark::subscription)
        .title("Hark")
        .run()
}
