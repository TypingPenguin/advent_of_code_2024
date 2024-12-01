use std::path::{absolute, Path};
use crate::helper_functions;
use log::{debug, info};
use std::env;
use crate::helper_functions::copy_to_clipboard;
// const MODE: &str = "test_1";
// const MODE: &str = "final_1";

// const MODE: &str = "test_2";
const MODE: &str = "final_2";

pub(crate) fn run() {
    //get current working directory
    let file = get_data();
    let mut final_value = String::new();

    // Run correct code based on mode
    match MODE {
        "test_1" => {log::set_max_level(log::LevelFilter::Debug);
            final_value = part_1(file);},
        "final_1" => {log::set_max_level(log::LevelFilter::Info);
            final_value = part_1(file);},
        "test_2" => {log::set_max_level(log::LevelFilter::Debug);
            final_value = part_2(file);},
        "final_2" => {log::set_max_level(log::LevelFilter::Debug);
            final_value = part_2(file);},
        _ => panic!("Invalid mode"),
    }

    // Try to copy the value to the clipboard
    match copy_to_clipboard(&final_value) {
        Ok(_) => info!("Successfully copied to clipboard: {}", final_value),
        Err(e) => eprintln!("Failed to copy to clipboard: {}", e),
    }

}

fn part_1(file:String) -> String {
    let mut final_value = String::new();
    // split the string on newlines
    let lines = file.split("\n");
    debug!("Raw_lines: {:?}", lines.clone().collect::<Vec<&str>>());

    // Remove the return lines, and the spaces that are too many
    let processed_lines: Vec<String> = lines
        .map(|line| line.replace("\r", "").replace("   ", " ")) // Replace unwanted characters
        .collect(); // Collect into a vector

    debug!("Lines after clean-up: {:?}", processed_lines.clone());


    // Split the columns into two vectors
    let mut get_sides: Vec<Vec<i32>> = vec![Vec::new(), Vec::new()];
    for line in processed_lines.iter() {
        let mut split_line = line.split(" ");
        // Put into correct side as integers
        get_sides[0].push(split_line.next().unwrap().parse::<i32>().unwrap());
        get_sides[1].push(split_line.next().unwrap().parse::<i32>().unwrap());
    }
    debug!("{:?}", get_sides);

    //Sort the two sides
    let sorted_sides = get_sides.iter().map(|side| {
        let mut side: Vec<i32> = side.iter().map(|&x| x).collect();
        side.sort();
        side
    }).collect::<Vec<Vec<i32>>>();
    debug!("{:?}", sorted_sides);

    // Compare the two sides
    let disparity = sorted_sides[0].iter().zip(sorted_sides[1].iter()).map(|(a, b)| {
        (a-b).abs()
    }).collect::<Vec<i32>>();
    debug!("Vector of disparities between the two sides {:?}", disparity);

    // Sum the disparity
    let sum: i32 = disparity.iter().sum();
    info!("Sum of disparities: {}", sum);

    final_value = sum.to_string();
    return final_value.to_string();
}

fn part_2(file:String) -> String {
    let mut final_value = String::new();
    // split the string on newlines
    let lines = file.split("\n");
    debug!("Raw_lines: {:?}", lines.clone().collect::<Vec<&str>>());

    // Remove the return lines, and the spaces that are too many
    let processed_lines: Vec<String> = lines
        .map(|line| line.replace("\r", "").replace("   ", " ")) // Replace unwanted characters
        .collect(); // Collect into a vector

    debug!("Lines after clean-up: {:?}", processed_lines.clone());


    // Split the columns into two vectors
    let mut get_sides: Vec<Vec<i32>> = vec![Vec::new(), Vec::new()];
    for line in processed_lines.iter() {
        let mut split_line = line.split(" ");
        // Put into correct side as integers
        get_sides[0].push(split_line.next().unwrap().parse::<i32>().unwrap());
        get_sides[1].push(split_line.next().unwrap().parse::<i32>().unwrap());
    }
    debug!("{:?}", get_sides);

    //Sum the multiplication of the left number by the amount of times it appears in the right side
    let same_number:i32 = get_sides[0].iter().map(|&x|{
        let count = get_sides[1].iter().filter(|&y| *y == x).count();
        debug!("Number {} appears {} times in the right side", x, count);
        count as i32 * x
    }).sum();
    info!("Sum of the multiplication of the left number by the amount of times it appears in the right side: {}", same_number);

    final_value = same_number.to_string();
    final_value
}

fn get_data() -> String {
    //get current directory of this file
    let cd = get_source_file_dir();

    // Return the file based on the mode
    let file = match MODE {
        "test_1" => helper_functions::load_txt_file(cd.to_owned() + "/data_example_1.txt"),
        "test_2" => helper_functions::load_txt_file(cd.to_owned() + "/data_example_2.txt"),
        "final_1" => helper_functions::load_txt_file(cd.to_owned() + "/data_final_1.txt"),
        "final_2" => helper_functions::load_txt_file(cd.to_owned() + "/data_final_2.txt"),
        _ => panic!("Invalid mode"),
    };

    return file;
}



fn get_source_file_dir() -> String {
    let source_file_path = file!(); // This gives the relative path to the source file
    let source_dir = Path::new(source_file_path).parent().unwrap();
    source_dir.to_str().unwrap().to_string()
}