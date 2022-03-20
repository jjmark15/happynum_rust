use crate::application::ApplicationService;
use crate::ports::cli::clap::run_cli;

mod application;
pub(crate) mod domain;
mod ports;

#[derive(Default)]
pub struct App;

impl App {
    pub fn new() -> Self {
        App::default()
    }

    pub fn run(&self) {
        let application_service = ApplicationService::new();
        run_cli(&application_service);
    }
}
