use std::collections::HashMap;
use std::path::{absolute, is_separator, Path};
use crate::helper_functions;
use log::{debug, info, warn};
use std::env;
use std::thread::sleep;
use image::{Rgb, RgbImage};
use log::Level::Debug;
use crate::helper_functions::copy_to_clipboard;
// const MODE: &str = "test_1";
// const MODE: &str = "final_1";
// const MODE: &str = "test_2";
const MODE: &str = "final_2";

pub(crate) fn run() {
    //get current working directory
    let file = get_data();
    let mut final_value = String::new();
    let now = std::time::Instant::now();


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

const SIZE_ARRAY: (isize, isize) = (101, 103); //wide, tall when viewed from above
const TIME: isize = 1;

fn part_1(file:String) -> String {

    let mut quadrants = vec![0,0,0,0];
    let mut final_value = String::new();
    // split the string on newlines
    let lines = file.split("\n").map(|x| x.trim()).collect::<Vec<&str>>();
    debug!("Raw_lines: {:?}", lines.clone());

    let new_lines = lines.iter().map(|x| x.replace("p=","").replace("v=","").split(" ").map(|x| x.split(",").map(|y| y.parse::<isize>().unwrap()).collect::<Vec<isize>>()).collect::<Vec<Vec<isize>>>()).collect::<Vec<Vec<Vec<isize>>>>();
    debug!("New_lines: {:?}", new_lines.clone());

    //make array of size SIZE_ARRAY for each robot
    let mut array = vec![vec![0; SIZE_ARRAY.1 as usize]; SIZE_ARRAY.0 as usize];

    for (i, line) in new_lines.iter().enumerate() {
        debug!("Line: {:?}", line.clone());
        let start_point = (line[0][0], line[0][1]);
        let velocity = (line[1][0], line[1][1]);

        let end_point = (start_point.0 + velocity.0 * TIME, start_point.1 + velocity.1 * TIME);
        let mut end_point_modulo = (end_point.0 % SIZE_ARRAY.0, end_point.1 % SIZE_ARRAY.1);
        if end_point_modulo.0 < 0 {
            end_point_modulo.0 += SIZE_ARRAY.0;
        }
        if end_point_modulo.1 < 0 {
            end_point_modulo.1 += SIZE_ARRAY.1;
        }
        debug!("Start_point: {:?}, Velocity: {:?}, End_point: {:?}, End_point_modulo: {:?}", start_point, velocity, end_point, end_point_modulo);
        array[end_point_modulo.0 as usize][end_point_modulo.1 as usize] += 1;


        debug!("Size_array: {:?}", SIZE_ARRAY.0/2);
        debug!("Size_array 2: {:?}", SIZE_ARRAY.1/2);

        if end_point_modulo.0 < (SIZE_ARRAY.0) / 2 && end_point_modulo.1 < (SIZE_ARRAY.1) / 2 {
            quadrants[0] += 1;
        } else if end_point_modulo.0 > (SIZE_ARRAY.0) / 2 && end_point_modulo.1 < (SIZE_ARRAY.1) / 2 {
            quadrants[1] += 1;
        } else if end_point_modulo.0 < (SIZE_ARRAY.0) / 2 && end_point_modulo.1 > (SIZE_ARRAY.1) / 2 {
            quadrants[2] += 1;
        } else if end_point_modulo.0 > (SIZE_ARRAY.0) / 2 && end_point_modulo.1 > (SIZE_ARRAY.1) / 2 {
            quadrants[3] += 1;
        }
    }

    pretty_print(&array);

    debug!("Quadrants: {:?}", quadrants);


    let mut product = 1; // Initialize product as 1

    for &num in &quadrants {
        product *= num; // Multiply each element
    }

    final_value = product.to_string();




    final_value
}

fn pretty_print(array: &Vec<Vec<isize>>) {
    let mut final_value = String::new();
    for y in 0..SIZE_ARRAY.1 {
        for x in 0..SIZE_ARRAY.0 {
            final_value.push_str(&array[x as usize][y as usize].to_string());
        }
        final_value.push_str("\n");
    }
    info!("Final_value: \n{}", final_value);
}

fn pretty_print_tree(array: &Vec<Vec<isize>>) -> String {
    let mut final_value = String::new();
    for y in 0..SIZE_ARRAY.1 {
        for x in 0..SIZE_ARRAY.0 {
            if array[x as usize][y as usize] == 0 {
                final_value.push_str(".");
            } else {
                final_value.push_str("#");
            }
        }
        final_value.push_str("\n");
    }
    debug!("Final_value: \n{}", final_value);
    final_value.to_string()
}

fn part_2(file:String) -> String {

    let mut quadrants = vec![0,0,0,0];
    let mut final_value = String::new();
    // split the string on newlines
    let lines = file.split("\n").map(|x| x.trim()).collect::<Vec<&str>>();
    debug!("Raw_lines: {:?}", lines.clone());

    let mut new_lines = lines.iter().map(|x| x.replace("p=","").replace("v=","").split(" ").map(|x| x.split(",").map(|y| y.parse::<isize>().unwrap()).collect::<Vec<isize>>()).collect::<Vec<Vec<isize>>>()).collect::<Vec<Vec<Vec<isize>>>>();
    debug!("New_lines: {:?}", new_lines.clone());

    //make array of size SIZE_ARRAY for each robot
    let mut array = vec![vec![0; SIZE_ARRAY.1 as usize]; SIZE_ARRAY.0 as usize];

    let mut new_time = 0;
    while true{
        let mut array = vec![vec![0; SIZE_ARRAY.1 as usize]; SIZE_ARRAY.0 as usize];
        new_time += 1;
        for line in new_lines.iter_mut() {
            debug!("Line: {:?}", line.clone());
            let mut start_point = (line[0][0], line[0][1]);
            let velocity = (line[1][0], line[1][1]);

            let end_point = (start_point.0 + velocity.0 * TIME, start_point.1 + velocity.1 * TIME);
            let mut end_point_modulo = (end_point.0 % SIZE_ARRAY.0, end_point.1 % SIZE_ARRAY.1);
            if end_point_modulo.0 < 0 {
                end_point_modulo.0 += SIZE_ARRAY.0;
            }
            if end_point_modulo.1 < 0 {
                end_point_modulo.1 += SIZE_ARRAY.1;
            }
            debug!("Start_point: {:?}, Velocity: {:?}, End_point: {:?}, End_point_modulo: {:?}", start_point, velocity, end_point, end_point_modulo);
            array[end_point_modulo.0 as usize][end_point_modulo.1 as usize] = 1;

            line[0][0] = end_point_modulo.0;
            line[0][1] = end_point_modulo.1;
        }

        info!("Time: {}", new_time);
        let data = pretty_print_tree(&array);

        for i in 0..SIZE_ARRAY.0 {
            for j in 0..SIZE_ARRAY.1 {
                if array[i as usize][j as usize] == 0 {
                    flood_fill(&mut array, j as usize, i as usize, 0, 1);
                    break;
                }
            }
            break;
        }

        let sum = array.iter().map(|x| x.iter().sum::<isize>()).sum::<isize>();
        if sum > 10000 || sum < 5000{
            continue;
        }

        // Generate the image from array-like data
        let img = array_to_image(array, SIZE_ARRAY.0 as u32, SIZE_ARRAY.1 as u32);

        // Save the generated image to a file
        save_image(img, new_time);
        if new_time == 500 {
            break;
        }


    }


    debug!("Quadrants: {:?}", quadrants);


    let mut product = 1; // Initialize product as 1

    for &num in &quadrants {
        product *= num; // Multiply each element
    }

    final_value = product.to_string();


    final_value
}



fn flood_fill(grid: &mut Vec<Vec<isize>>, x: usize, y: usize, target_value: i32, new_value: i32) {
    // Check bounds
    if x < 0 || x >= grid.len() || y < 0 || y >= grid[x].len() {
        return;
    }

    // If the current cell's value is not equal to the target value, do nothing
    if grid[x][y] != target_value.try_into().unwrap() {
        return;
    }

    // Fill the current cell with the new value
    grid[x][y] = new_value as isize;

    // 4-way flood fill: Recursively fill the adjacent cells (up, down, left, right)
    if x > 0 { // Check up
        flood_fill(grid, x - 1, y, target_value, new_value);
    }
    if x < grid.len() - 1 { // Check down
        flood_fill(grid, x + 1, y, target_value, new_value);
    }
    if y > 0 { // Check left
        flood_fill(grid, x, y - 1, target_value, new_value);
    }
    if y < grid[x].len() - 1 { // Check right
        flood_fill(grid, x, y + 1, target_value, new_value);
    }
}

fn array_to_image(array: Vec<Vec<isize>>, width: u32, height: u32) -> RgbImage {
    let mut img = RgbImage::new(array[0].len() as u32 +1, array.len() as u32 +1);

    for (y, row) in array.iter().enumerate() {
        for (x, &value) in row.iter().enumerate() {
            // Set color based on the value in the array
            let color = if value > 0 {
                Rgb([255, 255, 255])  // White for > 0
            } else {
                Rgb([0, 0, 0])  // Black for 0
            };

            // Set the pixel at (x, y)
            img.put_pixel(x as u32, y as u32, color);
        }
    }

    img
}

fn save_image(img: RgbImage, counter: u32) {
    // Save the image to a file (e.g., "output.png")
    let path = format!("images/output{}.png", counter);
    img.save(path).unwrap();
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