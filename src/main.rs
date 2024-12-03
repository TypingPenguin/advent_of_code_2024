use log::debug;

mod helper_functions;
mod template;
mod day_1;
mod day_3;

fn main() {
    // Initialize the logger
    env_logger::builder().filter_level(log::LevelFilter::Debug).init();

    // day_1::day::run();

    day_3::day::run();

}
