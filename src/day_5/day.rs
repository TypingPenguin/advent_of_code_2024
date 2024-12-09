use std::collections::HashMap;
use std::path::{absolute, Path};
use crate::helper_functions;
use log::{debug, info};
use std::env;
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
    let lines = file.split("\n").map(|x| x.trim()).collect::<Vec<&str>>();
    debug!("Raw_lines: {:?}", lines.clone());
    let mut rules = vec![];
    let mut updates: Vec<Vec<isize>> = vec![];
    let mut after_split = false;
    for item in lines.iter(){
        if *item == ""{
            after_split = true;
        }else if !after_split{
            rules.push(item.split("|").map(|x| x.parse::<isize>().unwrap()).collect::<Vec<isize>>());
        }else{
            updates.push(item.split(",").map(|x| x.parse::<isize>().unwrap()).collect::<Vec<isize>>());
        }
    }
    debug!("Rules: {:?}", rules);
    debug!("Updates: {:?}", updates);

    // This map will tell you which numbers are not allowed to be after the key number
    let mut hashmap: HashMap<isize, Vec<isize>> = std::collections::HashMap::new();
    for rule in rules.iter(){
        if hashmap.contains_key(&rule[1]) {
            hashmap.get_mut(&rule[1]).unwrap().push(rule[0]);
        } else {
            hashmap.insert(rule[1], vec![rule[0]]);
        }
    }
    debug!("Hashmap: {:?}", hashmap);

    // for rule in rules.iter(){
    //     let mut rule_vec = vec![];
    //     // add first element
    //     rule_vec.push(rule[1].clone());
    //     debug!("Rule_vec: {:?} for rule: {:?}", rule_vec, rule[0]);
    //     recursion_search_next_elements(&mut rule_vec, &rules, &rule[1]);
    //     debug!("Rule_vec at end: {:?}", rule_vec);
    //     hashmap.insert(rule[0], rule_vec);
    // }
    // debug!("Hashmap: {:?}", hashmap);

    let mut correct_updates = vec![];

    for update in updates.iter(){
        let mut correct = true;
        for i in 0..update.len()-1{
            if hashmap.contains_key(&update[i]){
                for x in i..update.len(){
                    if hashmap.get(&update[i]).unwrap().contains(&update[x]){
                        correct = false;
                        break;
                    }
                }
            }
        }
        if correct{
            correct_updates.push(update.clone());
        }
    }

    debug!("Correct updates: {:?}", correct_updates);

    //find middle element of each of the correct updates
    let mut middle_elements = vec![];
    for update in correct_updates.iter(){
        middle_elements.push(update[(update.len()-1)/2].clone());
    }
    debug!("Middle elements: {:?}", middle_elements);

    final_value = middle_elements.iter().sum::<isize>().to_string();


    final_value
}

fn recursion_search_next_elements(vector: &mut Vec<isize>, rules: &Vec<Vec<isize>>, rule_1: &isize) {
    for rule_2 in rules.iter(){
        if *rule_1 == rule_2[0]{
            vector.push(rule_2[1].clone());
            debug!("Rule vec: {:?}", vector);
            recursion_search_next_elements(vector, rules, &rule_2[1]);
        }
    }
}

fn part_2(file:String) -> String {
    let mut final_value = String::new();
    // split the string on newlines
    let lines = file.split("\n").map(|x| x.trim()).collect::<Vec<&str>>();
    debug!("Raw_lines: {:?}", lines.clone());
    let mut rules = vec![];
    let mut updates: Vec<Vec<isize>> = vec![];
    let mut after_split = false;
    for item in lines.iter(){
        if *item == ""{
            after_split = true;
        }else if !after_split{
            rules.push(item.split("|").map(|x| x.parse::<isize>().unwrap()).collect::<Vec<isize>>());
        }else{
            updates.push(item.split(",").map(|x| x.parse::<isize>().unwrap()).collect::<Vec<isize>>());
        }
    }
    debug!("Rules: {:?}", rules);
    debug!("Updates: {:?}", updates);

    // This map will tell you which numbers are not allowed to be after the key number
    let mut hashmap: HashMap<isize, Vec<isize>> = std::collections::HashMap::new();
    for rule in rules.iter(){
        if hashmap.contains_key(&rule[1]) {
            hashmap.get_mut(&rule[1]).unwrap().push(rule[0]);
        } else {
            hashmap.insert(rule[1], vec![rule[0]]);
        }
    }
    debug!("Hashmap: {:?}", hashmap);

    // for rule in rules.iter(){
    //     let mut rule_vec = vec![];
    //     // add first element
    //     rule_vec.push(rule[1].clone());
    //     debug!("Rule_vec: {:?} for rule: {:?}", rule_vec, rule[0]);
    //     recursion_search_next_elements(&mut rule_vec, &rules, &rule[1]);
    //     debug!("Rule_vec at end: {:?}", rule_vec);
    //     hashmap.insert(rule[0], rule_vec);
    // }
    // debug!("Hashmap: {:?}", hashmap);

    let mut correct_update_og = vec![];
    let mut correct_updates = vec![];
    let mut incorrect_updates = vec![];

    for update in updates.iter(){
        check_correct(&mut hashmap, &mut correct_update_og, update);
        check_correct_part_2(&mut hashmap, &mut correct_updates, &mut incorrect_updates, update, &mut true);
    }

    debug!("Correct updates og: {:?}", correct_update_og);
    debug!("Correct updates: {:?}", correct_updates);
    debug!("Incorrect updates: {:?}", incorrect_updates);

    //find middle element of each of the correct updates
    let mut middle_elements = vec![];
    for update in incorrect_updates.iter(){
        middle_elements.push(update[(update.len()-1)/2].clone());
    }
    debug!("Middle elements: {:?}", middle_elements);

    final_value = middle_elements.iter().sum::<isize>().to_string();


    final_value
}

fn check_correct(hashmap: &mut HashMap<isize, Vec<isize>>, correct_updates: &mut Vec<Vec<isize>>, update: &Vec<isize>) {
    let mut correct = true;
    for i in 0..update.len() - 1 {
        if hashmap.contains_key(&update[i]) {
            for x in i..update.len() {
                if hashmap.get(&update[i]).unwrap().contains(&update[x]) {
                    correct = false;
                    break;
                }
            }
        }
    }
    if correct {
        correct_updates.push(update.clone());
    }
}





fn check_correct_part_2(hashmap: &mut HashMap<isize, Vec<isize>>, correct_updates: &mut Vec<Vec<isize>>, incorrect_updates: &mut Vec<Vec<isize>>, update: &Vec<isize>, first_correct: &mut bool) {
    debug!("Checking update: {:?}", update);
    let mut correct = true;
    for i in 0..update.len() - 1 {
        if hashmap.contains_key(&update[i]) {
            for x in i..update.len() {
                if hashmap.get(&update[i]).unwrap().contains(&update[x]) {
                    correct = false;
                    *first_correct = false;

                    //swap the two values that break the rules
                    let mut new_update = update.clone().to_vec();
                    let i_value = new_update.get(i).unwrap().clone();
                    let x_value = new_update.get(x).unwrap().clone();
                    new_update[i] = x_value;
                    new_update[x] = i_value;

                    check_correct_part_2(hashmap, correct_updates,incorrect_updates, &new_update, first_correct);
                    return;
                }
            }
        }
    }
    if correct && *first_correct {
        correct_updates.push(update.clone());
    }
    if correct && !*first_correct{
        incorrect_updates.push(update.clone());
    }
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