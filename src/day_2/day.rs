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
        "final_2" => {log::set_max_level(log::LevelFilter::Info);
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
    let lines = file.split("\n").collect::<Vec<&str>>().iter().map(|x| x.trim()).collect::<Vec<&str>>();
    debug!("Raw_lines: {:?}", lines);

    // split the string on spaces and convert to integers
    let numbers = lines.iter().map(|x| x.split(" ").collect::<Vec<&str>>().iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>();
    debug!("Numbers: {:?}", numbers);

    // calculate the diff between the element and the next one
    let diff_table = numbers.iter().map(|line_number| {
        line_number.iter().zip(line_number.iter().skip(1)).map(|(a, b)| a - b).collect::<Vec<i32>>()
    }).collect::<Vec<Vec<i32>>>();
    debug!("Diff_table: {:?}", diff_table);

    let mut safe_counter = 0;

    for line in diff_table.iter() {
        let mut safe = true;
        let mut increasing = false;
        let mut decreasing = false;
        for diff in line.iter() {
            if *diff > 0 {
                increasing = true;
            }
            if *diff < 0 {
                decreasing = true;
            }
            if diff.abs() > 3 || diff.abs() < 1 {
                safe = false;
                break;
            }
        }
        if safe && (increasing ^ decreasing){
            safe_counter += 1;
        }
    }

    final_value = safe_counter.to_string();
    info!("Safe counter: {}", safe_counter);

    final_value
}

fn part_2(file:String) -> String {
    let mut final_value = String::new();
    // split the string on newlines
    let lines = file.split("\n").collect::<Vec<&str>>().iter().map(|x| x.trim()).collect::<Vec<&str>>();
    debug!("Raw_lines: {:?}", lines);

    // split the string on spaces and convert to integers
    let numbers = lines.iter().map(|x| x.split(" ").collect::<Vec<&str>>().iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>();
    debug!("Numbers: {:?}", numbers);

    // calculate the diff between the element and the next one
    let diff_table = numbers.iter().map(|line_number| {
        line_number.iter().zip(line_number.iter().skip(1)).map(|(a, b)| a - b).collect::<Vec<i32>>()
    }).collect::<Vec<Vec<i32>>>();
    debug!("Diff_table: {:?}", diff_table);

    //initialize the safe counter
    let mut safe_counter = 0;

    //for every line check if safe, if not, remove one element and check again
    for number in numbers.iter() {
        //if the line is safe, increment the counter, no need to do more
        if is_safe(number) {
            safe_counter += 1;
            debug!("Safe line: {:?}", number)
        }
        else {
            //for every combination of the line after removing one element
            for i in 0..number.len() {
                let mut new_line = number.clone();
                new_line.remove(i);
                if is_safe(&new_line) {
                    safe_counter += 1;
                    debug!("Safe line: {:?}", new_line);
                    break;
                }
            }
            debug!("Unsafe line: {:?}", number)
        }
    }

    final_value = safe_counter.to_string();
    final_value
}

fn is_safe(line: &Vec<i32>) -> bool {
    let mut safe = true;
    let mut increasing = false;
    let mut decreasing = false;
    let diff_line = line.iter().zip(line.iter().skip(1)).map(|(a, b)| a - b).collect::<Vec<i32>>();
    for diff in diff_line.iter() {
        if *diff > 0 {
            increasing = true;
        }
        if *diff < 0 {
            decreasing = true;
        }
        if diff.abs() > 3 || diff.abs() < 1 {
            safe = false;
            break;
        }
    }
    debug!("Line: {:?}, Safe: {}, Increasing: {}, Decreasing: {}", line, safe, increasing, decreasing);
    return safe && (increasing ^ decreasing);
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