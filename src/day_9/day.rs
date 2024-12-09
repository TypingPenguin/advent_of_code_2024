use std::collections::HashMap;
use std::path::{absolute, Path};
use crate::helper_functions;
use log::{debug, info};
use std::env;
use log::Level::Debug;
use crate::helper_functions::copy_to_clipboard;
// const MODE: &str = "test_1";
// const MODE: &str = "final_1";
//
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

    let matrix = lines.iter().map(|x| x.split("").filter(|x| *x!= "").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
    debug_matrix(&matrix);

    let mut node_matrix = vec![vec![0; matrix[0].len()]; matrix.len()];

    //make hashmap of all the nodes
    let mut nodes: HashMap<&str, Vec<[usize;2]>> = std::collections::HashMap::new();
    for (i, row) in matrix.iter().enumerate(){
        for (j, node) in row.iter().enumerate(){
            if *node != "."{
                if nodes.contains_key(node){
                    let mut location_list: &mut Vec<[usize; 2]> = nodes.get_mut(node).unwrap();
                    location_list.push([i, j]);
                } else {
                    nodes.insert(*node, vec![[i, j]]);
                }
            }
        }
    }

    debug!("Nodes: {:?}", nodes);


    //Find vector between two nodes
    let mut antinodes: HashMap<[usize;2], _> = std::collections::HashMap::new();
    for (node, locations) in nodes.iter(){
        for i in 0..locations.len(){
            for j in i+1..locations.len(){
                let node1 = locations[i];
                let node2 = locations[j];
                let vector = [node2[0] as isize - node1[0] as isize, node2[1] as isize - node1[1] as isize];
                let antinode1 = [node1[0] as isize - vector[0], node1[1] as isize - vector[1]];
                let antinode2 = [node2[0] as isize + vector[0], node2[1] as isize + vector[1]];
                if antinode1[0] >= 0 && antinode1[0] < matrix.len() as isize && antinode1[1] >= 0 && antinode1[1] < matrix[0].len() as isize{
                    debug!("Antinode1: {:?}", antinode1);
                    antinodes.insert([antinode1[0] as usize, antinode1[1] as usize], vector);
                }
                if antinode2[0] >= 0 && antinode2[0] < matrix.len() as isize && antinode2[1] >= 0 && antinode2[1] < matrix[0].len() as isize{
                    debug!("Antinode2: {:?}", antinode2);
                    antinodes.insert([antinode2[0] as usize, antinode2[1] as usize], vector);
                }

            }
        }
    }

    debug!("Antinodes: {:?}", antinodes.keys());
    debug!("Amount of antinodes: {:?}", antinodes.len());

    final_value = antinodes.len().to_string();







    final_value
}
fn debug_matrix(matrix: &Vec<Vec<&str>>){
    debug!("--------------------------Matrix---------------------------------");
    for row in matrix.iter(){
        let mut line: String = String::new();
        for node in row.iter(){
            line.push_str(node);
        }
        debug!("{:?}", line);
    }
}

fn part_2(file:String) -> String {
    let mut final_value = String::new();
    // split the string on newlines
    let lines = file.split("\n").map(|x| x.trim()).collect::<Vec<&str>>();
    debug!("Raw_lines: {:?}", lines.clone());

    let matrix = lines.iter().map(|x| x.split("").filter(|x| *x!= "").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
    debug_matrix(&matrix);

    let mut node_matrix = vec![vec![0; matrix[0].len()]; matrix.len()];

    //make hashmap of all the nodes
    let mut nodes: HashMap<&str, Vec<[usize;2]>> = std::collections::HashMap::new();
    for (i, row) in matrix.iter().enumerate(){
        for (j, node) in row.iter().enumerate(){
            if *node != "."{
                if nodes.contains_key(node){
                    let mut location_list: &mut Vec<[usize; 2]> = nodes.get_mut(node).unwrap();
                    location_list.push([i, j]);
                } else {
                    nodes.insert(*node, vec![[i, j]]);
                }
            }
        }
    }

    debug!("Nodes: {:?}", nodes);


    //Find vector between two nodes
    let mut antinodes: HashMap<[usize;2], _> = std::collections::HashMap::new();
    for (node, locations) in nodes.iter(){
        for i in 0..locations.len(){
            for j in i+1..locations.len(){
                let mut node1 = locations[i];
                let mut node2 = locations[j];
                let vector = [node2[0] as isize - node1[0] as isize, node2[1] as isize - node1[1] as isize];


                let self_node = [node1[0] as isize + vector[0], node1[1] as isize + vector[1]];
                let self_node2 = [node2[0] as isize - vector[0], node2[1] as isize - vector[1]];

                antinodes.insert([self_node[0] as usize, self_node[1] as usize], vector);
                antinodes.insert([self_node2[0] as usize, self_node2[1] as usize], vector);

                //for direction 1
                while true {
                    let antinode1 = [node1[0] as isize - vector[0], node1[1] as isize - vector[1]];
                    if antinode1[0] >= 0 && antinode1[0] < matrix.len() as isize && antinode1[1] >= 0 && antinode1[1] < matrix[0].len() as isize{
                        debug!("Antinode1: {:?}", antinode1);
                        antinodes.insert([antinode1[0] as usize, antinode1[1] as usize], vector);
                    } else { break; }
                    node1 = [antinode1[0] as usize, antinode1[1] as usize];
                }
                while true {
                    let antinode2 = [node2[0] as isize + vector[0], node2[1] as isize + vector[1]];
                    if antinode2[0] >= 0 && antinode2[0] < matrix.len() as isize && antinode2[1] >= 0 && antinode2[1] < matrix[0].len() as isize{
                        debug!("Antinode2: {:?}", antinode2);
                        antinodes.insert([antinode2[0] as usize, antinode2[1] as usize], vector);
                    } else { break; }
                    node2 = [antinode2[0] as usize, antinode2[1] as usize];

                }
            }
        }
    }
    let mut array_2d = vec![vec!["."; matrix[0].len()]; matrix.len()];
    for (key, value) in antinodes.iter(){
        array_2d[key[0]][key[1]] = "#";
    }
    debug_matrix(&array_2d);

    debug!("Antinodes: {:?}", antinodes.keys());
    debug!("Amount of antinodes: {:?}", antinodes.len());

    final_value = antinodes.len().to_string();







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