use log::debug;

mod helper_functions;
mod template;
mod day_1;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

fn main() {
    // Initialize the logger
    env_logger::builder().filter_level(log::LevelFilter::Debug).init();

    // day_1::day::run();

    // day_3::day::run();

    // day_4::day::run();

    // day_5::day::run();
    day_6::day::run();
    // day_7::day::run();
    // day_8::day::run();
    // day_9::day::run();
}
