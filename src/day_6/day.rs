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
    //start timer
    let now = std::time::Instant::now();

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

    //print how long it took
    let elapsed = now.elapsed();
    info!("Time it took to run: {}.{}", elapsed.as_secs(), elapsed.subsec_millis());

}


fn part_1(file:String) -> String {
    let direction: Vec<Vec<isize>> = vec![vec![-1, 0], vec![0, 1], vec![1, 0], vec![0, -1]];


    let mut final_value = String::new();
    // split the string on newlines
    let array: Vec<Vec<&str>> = file
        .split("\n")
        .map(|x| x.trim())
        .map(|line| line.split("").filter(|x| *x!= "").collect::<Vec<&str>>())
        .collect();
    debug!("Raw_lines: {:?}", array);

    let rows = array.len();
    let cols = array[0].len();

    let mut new_array = vec![vec![0; cols]; rows];

    //find start position in array where patrol is "^"
    let mut start_row = 0;
    let mut start_col = 0;


    for (row_index, row) in array.iter().enumerate(){
        for (col_index, char) in row.iter().enumerate(){
            if *char == "^"{
                start_row = row_index;
                start_col = col_index;
            }
        }
    }

    new_array[start_row][start_col] = 1;

    //patrol the array
    let mut row_pos:isize = start_row as isize;
    let mut col_pos:isize = start_col as isize;
    let mut direction_count = 0;

    let mut counter = 0;

    while true {
        row_pos += direction[direction_count][0];
        col_pos += direction[direction_count][1];
        if row_pos >= 0 && row_pos < rows as isize && col_pos >= 0 && col_pos < cols as isize {
            if array[row_pos as usize][col_pos as usize] == "#" {
                row_pos -= direction[direction_count][0];
                col_pos -= direction[direction_count][1];
                direction_count = (direction_count + 1) % 4;
            } else {
                new_array[row_pos as usize][col_pos as usize] = 1;
            }
        } else {
            break;
        }

    }

    let mut other_counter = 0;
    debug!("New_array: {:?}", new_array);
    for row in new_array.iter(){
        for char in row.iter(){
            if *char == 1 {
                counter += 1;
            }
        }
        debug!("Row: {:?}", row);
    }




    final_value = counter.to_string();

    final_value
}

fn part_2(file:String) -> String {
    let mut last_turn_positions: Vec<Vec<isize>> = vec![];
    let direction: Vec<Vec<isize>> = vec![vec![-1, 0], vec![0, 1], vec![1, 0], vec![0, -1]];


    let mut final_value = String::new();
    // split the string on newlines
    let array: Vec<Vec<&str>> = file
        .split("\n")
        .map(|x| x.trim())
        .map(|line| line.split("").filter(|x| *x!= "").collect::<Vec<&str>>())
        .collect();
    debug!("Raw_lines: {:?}", array);

    let rows = array.len();
    let cols = array[0].len();

    let mut new_array = vec![vec![0; cols]; rows];

    //find start position in array where patrol is "^"
    let mut start_row = 0;
    let mut start_col = 0;


    for (row_index, row) in array.iter().enumerate(){
        for (col_index, char) in row.iter().enumerate(){
            if *char == "^"{
                start_row = row_index;
                start_col = col_index;
            }
        }
    }

    new_array[start_row][start_col] = 1;

    //patrol the array


    let mut counter = 0;
    let mut loop_counter = 0;


    let mut row_pos:isize = start_row as isize;
    let mut col_pos:isize = start_col as isize;
    let mut direction_count = 0;
    while true {
        row_pos += direction[direction_count][0];
        col_pos += direction[direction_count][1];
        if row_pos >= 0 && row_pos < rows as isize && col_pos >= 0 && col_pos < cols as isize {
            if array[row_pos as usize][col_pos as usize] == "#" {
                row_pos -= direction[direction_count][0];
                col_pos -= direction[direction_count][1];
                direction_count = (direction_count + 1) % 4;
            } else {
                new_array[row_pos as usize][col_pos as usize] = 1;
            }
        } else {
            break;
        }

    }



    for i in 0..array.len() {
        for y in 0..array[i].len() {
            if new_array[i][y] == 0 {
                continue;
            }
            let mut loop_bool = true;
            if array[i][y] != "#" {
                let mut row_pos:isize = start_row as isize;
                let mut col_pos:isize = start_col as isize;
                let mut direction_count = 0;
                last_turn_positions = vec![];

                let mut array_clone = array.clone();
                array_clone[i][y] = "#";
                debug!("Array_clone: :");
                for row in array_clone.iter(){
                    debug!("Row: {:?}", row);
                }
                while loop_bool {
                    row_pos += direction[direction_count][0];
                    col_pos += direction[direction_count][1];
                    if row_pos >= 0 && row_pos < rows as isize && col_pos >= 0 && col_pos < cols as isize {
                        if array_clone[row_pos as usize][col_pos as usize] == "#" {
                            row_pos -= direction[direction_count][0];
                            col_pos -= direction[direction_count][1];
                            // debug!("Last_turn_positions: {:?}", last_turn_positions);
                            // debug!("Current position: [{}, {}]", row_pos, col_pos);
                            // debug!("Position of block: [{}, {}]", i, y);
                            if last_turn_positions.len() > 3 {
                                for x in 0..last_turn_positions.len(){
                                    if last_turn_positions[x][0] == row_pos && last_turn_positions[x][1] == col_pos && last_turn_positions[x][2] == direction_count as isize {
                                        debug!("Found loop!");
                                        loop_counter += 1;
                                        loop_bool = false;
                                        break;
                                    }
                                }

                            }
                            last_turn_positions.push(vec![row_pos, col_pos, direction_count as isize]);

                            direction_count = (direction_count + 1) % 4;
                        } else {
                            new_array[row_pos as usize][col_pos as usize] = 1;
                        }
                    } else {
                        break;
                    }

                }
            }
        }
    }


    let mut other_counter = 0;
    debug!("New_array: {:?}", new_array);
    for row in new_array.iter(){
        for char in row.iter(){
            if *char == 1 {
                counter += 1;
            }
        }
        debug!("Row: {:?}", row);
    }




    final_value = loop_counter.to_string();

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