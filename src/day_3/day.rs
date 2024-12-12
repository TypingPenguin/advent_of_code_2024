use std::path::{absolute, Path};
use crate::helper_functions;
use log::{debug, info};
use std::env;
use std::ptr::null_mut;
use crate::helper_functions::copy_to_clipboard;
use regex::Regex;

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

    //Parse the numbers with regex
    let re = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\))").unwrap();
    let mut results = vec![];
    let mut multiplactions = vec![];
    for (_, [full_match, nr_1,nr_2]) in re.captures_iter(&*file).map(|c| c.extract()) {
        multiplactions.push([nr_1.parse::<isize>().unwrap(), nr_2.parse::<isize>().unwrap()]);
        results.push(nr_1.parse::<isize>().unwrap() * nr_2.parse::<isize>().unwrap());
    }
    debug!("{:?}", results);

    let sum = results.iter().sum::<isize>();
    final_value = sum.to_string();
    final_value
}


#[derive(Debug)]
pub struct Match{
    mult: Vec<isize>,
    result: isize,
    position: usize,
}


fn part_2(file:String) -> String {
    let mut final_value = String::new();

    let mut matches: Vec<Match> = vec![];

    //Parse the numbers with regex and push them in matches
    let re = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\))").unwrap();
    for (captures, pos) in re.captures_iter(&*file).map(|c| (c.extract(), c.get(0).unwrap().start())) {
        let (_,[full_match, nr_1, nr_2]) = captures;
        matches.push(Match{
            mult: [nr_1.parse::<isize>().unwrap(), nr_2.parse::<isize>().unwrap()].to_vec(),
            result: nr_1.parse::<isize>().unwrap() * nr_2.parse::<isize>().unwrap(),
            position: pos,
        });
    }

    //Parse the DO and DON'T
    let re = Regex::new(r"(do)").unwrap(); //WHY IS LOOKAHEAD NOT IMPLEMENTED
    let mut do_pos_old = vec![0]; //We start with a do implicit
    do_pos_old.extend(re.captures_iter(&*file).map(|c| c.get(0).unwrap().start()));

    debug!("Do_pos_old: {:?}", do_pos_old);


    //Parse the DO and DON'T
    let re = Regex::new(r"(don't)").unwrap();
    let mut dont_pos = vec![];
    dont_pos.extend(re.captures_iter(&*file).map(|c| c.get(0).unwrap().start()));
    dont_pos.push(999999999); //add end don't because I don't want to deal with this

    debug!("Dont_pos{:?}", dont_pos);

    //Make sure do, was not recognized as substring of dont
    let mut do_pos = vec![];
    for i in 0..do_pos_old.len() {
        if !dont_pos.contains(&do_pos_old[i]) {
            do_pos.push(do_pos_old[i]);
        }
    }
    debug!("Do_pos: {:?}", do_pos);


    //Make the ranges where we do consider matches
    let do_range = do_pos.iter().map(|x| {
        dont_pos.iter()
            .find(|y| **y > *x) // Find the first `y` where `*y > *x`
            .map(|y| {
                debug!("{:?}", x);
                debug!("{:?}", y);
                [x, y]}).unwrap()  // Map it to `[x, y]` if found
    }).collect::<Vec<_>>();

    debug!("Do range: {:?}", do_range);


    let match_sum = matches.iter().map(|m| {
        do_range.iter().find(|range| (range[0] <= &m.position) && &m.position <= range[1]).map(|map| {
            m.result
        }).unwrap_or(0)
    }).sum::<isize>();


    debug!("Matches: {:?}", matches);
    debug!("Match of the sum{:?}", match_sum);

    final_value = match_sum.to_string();
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