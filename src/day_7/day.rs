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
#[derive(Debug)]
struct Data {
    result: isize,
    numbers: Vec<isize>,
}

fn generator(array: Vec<isize>) -> (Vec<Vec<char>>, Vec<isize>) {
    let operators = ['+', '*'];
    let mut results = Vec::new();
    let mut intermediate: isize = 0;
    let mut combined_number: isize = 0;
    let mut sum_vector = Vec::new();

    // Total combinations: 2^(length-1)
    let total_combinations = 1 << (array.len() - 1);

    for i in 0..total_combinations {
        intermediate = array[0];
        let mut result_vector = Vec::new();

        for j in 0..array.len() {
            if j < array.len() - 1 {
                let operator = if (i & (1 << j)) == 0 {
                    // in case of Addition
                    intermediate += array[j+1];
                    '+'
                } else {
                    // in case of Multiplication
                    intermediate *= array[j+1];
                    '*'
                };
                result_vector.push(operator);
            }
        }
        sum_vector.push(intermediate);
        results.push(result_vector);
    }

    // Print the results
    (results, sum_vector)
}
fn generator_2(array: Vec<isize>) -> (Vec<Vec<char>>) {
    let operators = ['+', '*', '|'];
    let mut results = Vec::new();

    // Total combinations: 3^(length-1)
    let total_combinations = usize::pow(operators.len(), (array.len() - 1) as u32);

    for i in 0..total_combinations {
        let mut intermediate = array[0];
        let mut result_vector = Vec::new();
        let mut current = i;

        for j in 0..array.len() - 1 {
            // Determine the operator using base-3 logic
            let operator_index = current % operators.len();
            current /= operators.len();

            // Store the operator in the result vector
            result_vector.push(operators[operator_index]);
        }

        results.push(result_vector);
    }

    // Return the operator combinations and their corresponding results
    results
}



fn part_1(file:String) -> String {
    let mut final_value = String::new();
    // split the string on newlines
    let lines = file.split("\n").map(|x| x.trim()).collect::<Vec<&str>>();
    debug!("Raw_lines: {:?}", lines.clone());

    let data_matrix = lines.iter().map(|x| {
        debug!("Line: {:?}", x);
        let mut result = 0;
        let mut spit_line = x.clone().split(":");
        result = spit_line.next().unwrap().parse::<isize>().unwrap();
        let numbers = spit_line.next().unwrap().split(" ").filter(|x| *x != "").map(|x| x.trim().parse::<isize>().unwrap()).collect::<Vec<isize>>();
        Data{result, numbers}
    }).collect::<Vec<Data>>();

    debug!("Data Matrix: {:?}", data_matrix);



    let mut counter = 0;
    for data in data_matrix.iter(){
        let (matrix, sum_vector) = generator(data.numbers.clone());
        debug!("Matrix: {:?}", matrix);
        debug!("Sum Vector: {:?}", sum_vector);
        for x in sum_vector.iter(){
            if *x == data.result{
                counter += *x;
                break;
            }
        }
    }



    final_value = counter.to_string();
    final_value
}

fn part_2(file:String) -> String {
    let mut final_value = String::new();
    // split the string on newlines
    let lines = file.split("\n").map(|x| x.trim()).collect::<Vec<&str>>();
    debug!("Raw_lines: {:?}", lines.clone());

    let data_matrix = lines.iter().map(|x| {
        debug!("Line: {:?}", x);
        let mut result = 0;
        let mut spit_line = x.clone().split(":");
        result = spit_line.next().unwrap().parse::<isize>().unwrap();
        let numbers = spit_line.next().unwrap().split(" ").filter(|x| *x != "").map(|x| x.trim().parse::<isize>().unwrap()).collect::<Vec<isize>>();
        Data{result, numbers}
    }).collect::<Vec<Data>>();

    debug!("Data Matrix: {:?}", data_matrix);



    let mut counter_total = 0;


    for data in data_matrix.iter(){
        if data.result == 7290 {
            debug!("------------------------------------Data: {:?}", data);
        }
        let matrix = generator_2(data.numbers.clone());
            for combination in matrix.iter() {
                let mut intermediate = data.numbers[0];
                if data.result == 7290 {

                    debug!("Combination: {:?}", combination);
                    debug!("Intermediate: {}", intermediate);
                }
                for (i, x) in combination.iter().enumerate() {
                    if *x == '+' {
                        debug!("Intermediate PLUS: {} + {}", intermediate , data.numbers[i + 1]);
                        intermediate += data.numbers[i + 1].clone();
                        debug!("Intermediate PLUS: {}", intermediate);
                    } else if *x == '*' {
                        debug!("Intermediate Multiple: {} * {}", intermediate , data.numbers[i + 1]);

                        intermediate *= data.numbers[i + 1].clone();
                        debug!("Intermediate Multiple: {}", intermediate);

                    } else if *x == '|' {
                        debug!("Intermediate COMB: {} | {}", intermediate , data.numbers[i + 1]);

                        let mut string_number = intermediate.to_string();
                        string_number.push_str(&data.numbers[i + 1].to_string());
                        intermediate = string_number.parse::<isize>().unwrap();
                        debug!("Intermediate COMB: {}", intermediate);

                    }
                }

                if intermediate == data.result {
                    debug!("Intermediate: {}", intermediate);
                    debug!("Combination: {:?}", combination);
                    counter_total += intermediate;
                    break;
                }
            }

    }


    final_value = counter_total.to_string();
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