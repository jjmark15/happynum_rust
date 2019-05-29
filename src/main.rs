#[macro_use]
extern crate clap;
extern crate happynum;

mod cli;

fn main() {
    cli::instantiate_cli();
}