mod assignment;
mod cli;
mod course;
mod utils;

use clap::ArgMatches;
use std::env;
use std::path::PathBuf;
use std::process::exit;

use crate::cli::add::add_run;
use crate::cli::new::new_run;
use crate::cli::remove::remove_run;
use crate::cli::root;
use crate::cli::show::show_run;
use crate::cli::update::update_run;
use crate::course::CourseList;


// Grades file
fn get_grades_file_path(matches: &ArgMatches) -> String {
    let mut file_path: PathBuf = home::home_dir().unwrap_or(PathBuf::from(""))
        .join(".courses.json");

    if let Ok(f) = env::var("COURSES_FILE") {
        file_path = PathBuf::from(f);
    }

    if let Some(f) = matches.get_one::<String>("file") {
        file_path = PathBuf::from((*f).clone());
    }

    file_path.into_os_string()
        .into_string().unwrap()
}

fn main() {
    let matches = root::root_command().get_matches();

    let grades_file_path = get_grades_file_path(&matches);
    if grades_file_path.is_empty() {
        eprintln!("Could not parse calc filename");
        exit(1);
    }

    let mut course_list = CourseList::load(&grades_file_path);

    match matches.subcommand() {
        Some(("list", _)) => println!("{course_list}"),
        Some(("new", new_matches)) => new_run(&new_matches, &mut course_list),
        Some(("add", add_matches)) => add_run(&add_matches, &mut course_list),
        Some(("update", update_matches)) => update_run(&update_matches, &mut course_list),
        Some(("show", show_matches)) => show_run(&show_matches, &mut course_list),
        Some(("remove", remove_matches)) => remove_run(&remove_matches, &mut course_list),
        Some((&_, _)) => {}
        None => {}
    };

    if let Err(err) = course_list.save(&grades_file_path) {
        eprintln!("Could not save grades file: {}", err);
        exit(1);
    }
}
