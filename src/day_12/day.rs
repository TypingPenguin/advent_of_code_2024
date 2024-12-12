use std::collections::HashMap;
use std::path::{absolute, Path};
use crate::helper_functions;
use log::{debug, info};
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

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn part_1(file:String) -> String {
    let mut final_value = String::new();
    // split the string on newlines
    let lines = file.split("\n").map(|x| x.trim()).collect::<Vec<&str>>();
    let elements = lines.iter().map(|x| x.split("").filter(|x| *x != "").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
    debug!("Raw_lines: {:?}", elements.clone());
    info!("Elements: {:?}", elements);

    let mut id_gardens = vec![vec![0; elements[0].len()]; elements.len()];
    let mut id = 1;

    for i in 0..elements.len() {
        for j in 0..elements[0].len() {
            if id_gardens[i][j] == 0{
                id_gardens[i][j] = id;
                search_around(&elements, elements[i][j].clone(), i, j, &mut id_gardens, id, 0);
                id += 1;
            }

        }
    }

    print_pretty_matrix(&id_gardens);
    debug!("Id_gardens: {:?}", id_gardens);


    //find area of each garden
    let mut garden_areas: HashMap<usize, i32> = HashMap::new();
    for id_iter in 1..id{
        let mut area = 0;
        for i in 0..id_gardens.len(){
            for j in 0..id_gardens[0].len(){
                if id_gardens[i][j] == id_iter{
                    area += 1;
                }
            }
        }
        garden_areas.insert(id_iter as usize, area);
    }

    debug!("Garden_areas: {:?}", garden_areas);

    // find perimeter of each garden
    let mut garden_perimeters: HashMap<usize, i32> = HashMap::new();
    for id_iter in 1..id{
        let mut perimeter = 0;
        for i in 0..id_gardens.len(){
            for j in 0..id_gardens[0].len(){
                if id_gardens[i][j] == id_iter{
                    for (k, l) in DIRECTIONS.iter(){
                        let new_i = i as i32 + k;
                        let new_j = j as i32 + l;
                        if new_i < 0 || new_i >= id_gardens.len() as i32 || new_j < 0 || new_j >= id_gardens[0].len() as i32{
                            perimeter += 1;
                        } else if id_gardens[new_i as usize][new_j as usize] != id_iter{
                            perimeter += 1;
                        }
                    }
                }
            }
        }
        garden_perimeters.insert(id_iter as usize, perimeter);
    }

    debug!("Garden_perimeters: {:?}", garden_perimeters);

    // find sum of multiplication between area and perimeter
    let mut garden_values: HashMap<usize, i32> = HashMap::new();
    for (id, area) in garden_areas.iter(){
        garden_values.insert(*id, area * garden_perimeters.get(id).unwrap());
    }

    debug!("Garden_values: {:?}", garden_values);

    let sum = garden_values.iter().map(|(id, value)| value).sum::<i32>();

    final_value = sum.to_string();


    final_value
}

fn print_pretty_matrix(matrix: &Vec<Vec<i32>>){
    for row in matrix.iter(){
        for element in row.iter(){
            print!("{:4}", element);
        }
        println!();
    }
}

fn search_around(elements: &Vec<Vec<&str>>, char: &str, i: usize, j: usize, id_gardens: &mut Vec<Vec<i32>>, id: i32, direction: i32){
    for (k, l) in DIRECTIONS.iter(){
        let new_i = i as i32 + k;
        let new_j = j as i32 + l;
        if new_i >= 0 && new_i < id_gardens.len() as i32 && new_j >= 0 && new_j < id_gardens[0].len() as i32{
            if id_gardens[new_i as usize][new_j as usize] == 0 && char == elements[new_i as usize][new_j as usize]{
                id_gardens[new_i as usize][new_j as usize] = id;
                search_around(&elements, char, new_i as usize, new_j as usize, id_gardens, id, 0);
            }
        }
    }


}

fn part_2(file:String) -> String {
    let mut final_value = String::new();
    // split the string on newlines
    let lines = file.split("\n").map(|x| x.trim()).collect::<Vec<&str>>();
    let elements = lines.iter().map(|x| x.split("").filter(|x| *x != "").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
    debug!("Raw_lines: {:?}", elements.clone());
    info!("Elements: {:?}", elements);

    let mut id_gardens = vec![vec![0; elements[0].len()]; elements.len()];
    let mut id = 1;

    for i in 0..elements.len() {
        for j in 0..elements[0].len() {
            if id_gardens[i][j] == 0{
                id_gardens[i][j] = id;
                search_around(&elements, elements[i][j].clone(), i, j, &mut id_gardens, id, 0);
                id += 1;
            }

        }
    }

    print_pretty_matrix(&id_gardens);
    debug!("Id_gardens: {:?}", id_gardens);


    //find area of each garden
    let mut garden_areas: HashMap<usize, i32> = HashMap::new();
    for id_iter in 1..id{
        let mut area = 0;
        for i in 0..id_gardens.len(){
            for j in 0..id_gardens[0].len(){
                if id_gardens[i][j] == id_iter{
                    area += 1;
                }
            }
        }
        garden_areas.insert(id_iter as usize, area);
    }

    debug!("Garden_areas: {:?}", garden_areas);


    //add zeros to the sides of id gardens to pad the matrix
    let mut id_gardens_padded = vec![vec![0; elements[0].len() + 2]; elements.len() + 2];
    for i in 0..id_gardens.len(){
        for j in 0..id_gardens[0].len(){
            id_gardens_padded[i + 1][j + 1] = id_gardens[i][j];
        }
    }
    info!("Padded matrix");
    print_pretty_matrix(&id_gardens_padded);


    // find corner count of each garden by using 2x2 slices of the matrix
    // A corner will be counted if the slice has 3 of the same id or 3 of different id
    // And diagonals will be counted as 2 corners
    /**
    0 0  <- 1 external corner.  By counting the amount of elements we can determine if it is an external or internal corner
    0 1

    1 1  <- 1 internal corner
    1 0

    1 0
    0 1  <- 2 external corners. This is a special case where the corners are counted twice, and checked by looking at diagonal values.

    **/

    let mut hashmap_corner_count:HashMap<usize,i32> = HashMap::new();
    let matrix_grow = [(1,0),(0,1),(1,1)];
    for id_iter in 1..id{
        let mut corner_count_id = 0;
        for i in 0..id_gardens_padded.len()-1{
            for j in 0..id_gardens_padded[0].len()-1{
                let mut count_same_id = 0;
                let mut count_other_id = 0;
                if id_gardens_padded[i][j] == id_iter{
                    count_same_id += 1;
                } else {
                    count_other_id += 1;
                }
                for direction in matrix_grow.iter(){
                    let new_i = i as i32 + direction.0;
                    let new_j = j as i32 + direction.1;
                    if id_gardens_padded[new_i as usize][new_j as usize] == id_iter{
                        count_same_id += 1;
                    } else {
                        count_other_id += 1;
                    }
                }

                debug!("ID: {} -- Location: {}:{}, has Count_same_id: {}, count_other_id: {}",id_iter,i,j,count_same_id, count_other_id);
                if count_same_id == 3 || count_other_id == 3{
                    corner_count_id += 1;
                }

                //handle the following diagonals as they add two corners each
                // 1 0
                // 0 1

                // 0 1
                // 1 0
                if count_same_id == 2{
                    if id_gardens_padded[i][j] == id_iter && id_gardens_padded[i+1][j+1] == id_iter{
                        corner_count_id += 2;
                    } else if id_gardens_padded[i+1][j] == id_iter && id_gardens_padded[i][j+1] == id_iter{
                        corner_count_id += 2;
                    }
                }
            }
        hashmap_corner_count.insert(id_iter as usize, corner_count_id);
        }
    }

    debug!("Corner count: {:?}", hashmap_corner_count);






    // find sum of multiplication between area and perimeter
    let mut garden_values: HashMap<usize, i32> = HashMap::new();
    for (id, area) in garden_areas.iter(){
        garden_values.insert(*id, area * hashmap_corner_count.get(id).unwrap());
    }

    debug!("Garden_values: {:?}", garden_values);

    let sum = garden_values.iter().map(|(id, value)| value).sum::<i32>();

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