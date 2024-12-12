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
    let directions: Vec<Vec<isize>> = vec![vec![-1,0],vec![1,0],vec![0,-1],vec![0,1],vec![-1,-1],vec![-1,1],vec![1,-1],vec![1,1]];


    let lines = file.split("\n").map(|x| x.trim()).collect::<Vec<&str>>();
    let matrix = lines.iter().map(|x| x.split("").filter(|char| *char != "").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
    debug!("Raw_lines: {:?}", lines);
    debug!("Matrix: {:?}", matrix);
    for line in matrix.iter(){
        debug!("Line: {:?}", line);
    }

    let mut counter = 0;
    for (row_index, line) in matrix.iter().enumerate(){
        for (col_index, char) in line.iter().enumerate(){
            if *char == "X"{
                for direction in directions.iter(){
                    let mut string = "X".to_string(); //Can make this prettier but not necessary for AOC
                    let mut row: isize = row_index as isize;
                    let mut col: isize = col_index as isize;
                    for i in 0..3{
                        row += direction[0];
                        col += direction[1];
                        if row < 0 || col < 0 || row >= matrix.len() as isize || col >= matrix[0].len() as isize{
                            break;
                        }
                        string.push(matrix[row as usize][col as usize].parse().unwrap());

                    }
                    debug!("String: {}", string);
                    if string == "XMAS"{
                        counter += 1;
                        debug!("Counter: {}", counter);
                    }
                }
            }
        }
    }


    final_value = counter.to_string();
    final_value
}

fn part_2(file:String) -> String {
    let mut final_value = String::new();
    // split the string on newlines
    let directions: Vec<Vec<isize>> = vec![vec![-1,-1,1,1],vec![-1,1,1,-1]];


    let lines = file.split("\n").map(|x| x.trim()).collect::<Vec<&str>>();
    let matrix = lines.iter().map(|x| x.split("").filter(|char| *char != "").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
    debug!("Raw_lines: {:?}", lines);
    debug!("Matrix: {:?}", matrix);
    for line in matrix.iter(){
        debug!("Line: {:?}", line);
    }

    let mut counter = 0;
    for (row_index, line) in matrix.iter().enumerate(){
        for (col_index, char) in line.iter().enumerate(){
            if *char == "A"{
                let mut counter_correct = 0;
                for direction in directions.iter(){

                    let char1_row = (row_index as isize + direction[0]) as isize;
                    let char1_col =  (col_index as isize + direction[1]) as isize;

                    let char2_row =  (row_index as isize + direction[2]) as isize;
                    let char2_col =  (col_index as isize + direction[3]) as isize;

                    if char1_row < 0 || char1_col < 0 || char1_row >= matrix.len() as isize || char1_col >= matrix[0].len() as isize{
                        break;
                    }
                    if char2_row < 0 || char2_col < 0 || char2_row >= matrix.len() as isize || char2_col >= matrix[0].len() as isize{
                        break;
                    }

                    let char1 = matrix[char1_row as usize][char1_col as usize];
                    let char2 = matrix[char2_row as usize][char2_col as usize];
                    if (char1 == "M" && char2 == "S") || (char1 == "S" && char2 == "M"){
                        counter_correct += 1;
                    }
                }
                if counter_correct == 2{
                    counter += 1;
                }
            }
        }
    }


    final_value = counter.to_string();
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