use crate::cli::run_cli;

mod cli;

pub mod happynum;

#[derive(Default)]
pub struct App;

impl App {
    pub fn new() -> Self {
        App::default()
    }

    pub fn run(&self) {
        run_cli();
    }
}
