use std::collections::HashMap;
use std::path::{absolute, Path};
use crate::helper_functions;
use log::{debug, error, info, warn};
use std::env;
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

fn part_1(file:String) -> String {
    let mut final_value = String::new();
    // split the string on newlines
    let lines = file.split("\n").map(|x| x.trim()).collect::<Vec<&str>>();
    debug!("Raw_lines: {:?}", lines.clone());

    let problem_array = format_array(lines);
    debug!("Problem_array: {:?}", problem_array.clone());

    let mut sum = 0;

    let mut i = 0;
    let length_of_problems = problem_array.len();
    for problem in problem_array{
        i += 1;
        info!("Problem number: {} out of {}", i, length_of_problems);
        let result = problem[2].clone();
        let button_a = problem[0].clone();
        let button_b = problem[1].clone();

        let leftover_x = result[0] % button_b[0];
        let leftover_y = result[1] % button_b[1];

        let mut token_array = Vec::new();

        //do naive impl.
        let mut max_b_x = result[0]/button_b[0];
        let mut max_b_y = result[1]/button_b[1];

        let max = max_b_x.max(max_b_y);


        debug!("Max: {}", max);
        for i in (0..=max).rev(){
            if result[0] < i*button_b[0] ||  i*button_b[1] > result[1]{
                continue;
            }
            let leftover_x = result[0] - i*button_b[0];
            let leftover_y = result[1] - i*button_b[1];

            if leftover_x % button_a[0] == 0 && leftover_y % button_a[1] == 0{
                let b_times = i;
                let a_times = leftover_x/button_a[0];
                if a_times >100 || b_times > 100{
                    continue;
                }
                if a_times*button_a[0] + b_times*button_b[0] != result[0] || a_times*button_a[1] + b_times*button_b[1] != result[1]{
                    warn!("Something went wrong");
                    warn!("a_times: {}", a_times);
                    warn!("b_times: {}", b_times);
                    warn!("Button_a: {:?}", button_a);
                    warn!("Button_b: {:?}", button_b);
                    warn!("Result: {:?}", result);
                    warn!("Problem: {:?}", problem);
                    continue;
                }
                let token_count = a_times*3 + b_times*1;
                token_array.push(token_count);
                info!("Result: {:?} for problem {:?}", (a_times,b_times), problem);

            }
        }

        if token_array.len() == 0{
            continue;
        }
        if token_array.len() > 1{
            error!("More than one solution");
        }
        let min_token = token_array.iter().min().unwrap();
        info!("Min_token: {}", min_token);
        sum += min_token;

    }

    final_value = sum.to_string();

    final_value
}

fn format_array(lines: Vec<&str>) -> Vec<Vec<Vec<usize>>>{
    let mut counter: i32 = 0;
    let mut array:Vec<Vec<Vec<usize>>> = Vec::new();
    let mut problem_vector:Vec<Vec<usize>> = Vec::new();
    for (i, line) in lines.iter().enumerate(){
        if i%4 == 0{
            problem_vector = Vec::new();
            let value_vector = line.split(":").skip(1).next().unwrap().split(",").map(|x| x.trim().split_at(2).1.parse::<usize>().unwrap()).collect::<Vec<usize>>();
            debug!("Value_vector: {:?}", value_vector);
            problem_vector.push(value_vector);
        } else if i%4 == 1{
            let value_vector = line.split(":").skip(1).next().unwrap().split(",").map(|x| x.trim().split_at(2).1.parse::<usize>().unwrap()).collect::<Vec<usize>>();
            debug!("Value_vector: {:?}", value_vector);
            problem_vector.push(value_vector);
        } else if i%4 == 2{
            let value_vector = line.split(":").skip(1).next().unwrap().split(",").map(|x| x.trim().split_at(2).1.parse::<usize>().unwrap()).collect::<Vec<usize>>();
            debug!("Value_vector: {:?}", value_vector);
            problem_vector.push(value_vector);
        } else if i%4 == 3{
            debug!("Problem_vector: {:?}", problem_vector.clone());
            debug!("Empty line");
            array.push(problem_vector.clone());
        }
    }
    return array;

}

fn part_2(file:String) -> String {
    let mut final_value = String::new();
    // split the string on newlines
    let lines = file.split("\n").map(|x| x.trim()).collect::<Vec<&str>>();
    debug!("Raw_lines: {:?}", lines.clone());

    let problem_array = format_array(lines);
    debug!("Problem_array: {:?}", problem_array.clone());

    let mut sum = 0;

    let mut i = 0;
    let length_of_problems = problem_array.len();
    for problem in problem_array{
        i += 1;
        info!("Problem number: {} out of {}", i, length_of_problems);
        let result = [problem[2][0] + 0, problem[2][1] + 0];
        let button_a = problem[0].clone();
        let button_b = problem[1].clone();

        //make it ax +by = C
        let mut a_1 = button_a[0] as isize;
        let mut b_1 = button_b[0] as isize;
        let mut c_1 = result[0] as isize;

        let mut a_2 = button_a[1] as isize;
        let mut b_2 = button_b[1] as isize;
        let mut c_2 = result[1] as isize;
        let a_1_scaled = a_1.clone();
        let a_2_scaled = a_2.clone();

        //find x and y
        a_1 = a_1 * a_2_scaled;
        b_1 = b_1 * a_2_scaled;
        c_1 = c_1 * a_2_scaled;


        a_2 = a_2 * a_1_scaled;
        b_2 = b_2 * a_1_scaled;
        c_2 = c_2 * a_1_scaled;

        let a_total = a_1 - a_2;
        let b_total = b_1 - b_2;
        let c_total = c_1 - c_2;


        if i == 2{
            info!("A: {}", a_total);
            info!("B: {}", b_total);
            info!("C: {}", c_total);
            info!("Y: {}", c_total/b_total);
        }

        //check if it is possible
        if c_total % b_total != 0{
            continue;
        }

        let y = c_total/b_total;

        let x = (result[0] - button_b[0]*y as usize)/button_a[0];
        debug!("X: {}", x);
        debug!("Y: {}", y);

        if x*button_a[0] + y as usize*button_b[0] != result[0] || x*button_a[1] + y as usize*button_b[1] != result[1]{
            warn!("Something went wrong");
            warn!("x: {}", x);
            warn!("y: {}", y);
            warn!("Button_a: {:?}", button_a);
            warn!("Button_b: {:?}", button_b);
            warn!("Result: {:?}", result);
            warn!("Problem: {:?}", problem);
            continue;
        }
        sum += x*3 + y as usize*1;

    }

    final_value = sum.to_string();

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