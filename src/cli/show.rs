use std::process::exit;

use clap::{Command, arg, ArgAction, ArgGroup, ArgMatches};

use crate::course::CourseList;

use super::root::course_list_arg;

pub fn show_command() -> Command {
    Command::new("show").alias("s")
        .about("Show grades of courses")
        .arg(arg!(-a --all <ALL> "Show all courses")
            .action(ArgAction::SetTrue))
        .arg(course_list_arg())
        .group(ArgGroup::new("COURSES_TO_SHOW")
            .args(["all", "COURSES"]))
}

pub fn show_run(matches: &ArgMatches, course_list: &mut CourseList) {
    if matches.get_flag("all") { show_all(course_list) }
    else { show_selected(matches, course_list) }
}

fn show_all(course_list: &CourseList) {
    (&course_list.0).into_iter().for_each(|(_, c)| {
        c.show_assignments();
        println!()
    });
}

fn show_selected(matches: &ArgMatches, course_list: &mut CourseList) {
    for course_name in matches.get_many::<String>("COURSES").unwrap() {
        let course = course_list.find_course(course_name);
        if course.is_none() {
            eprintln!("Course {} not found", course_name);
            exit(1);
        }

        course.unwrap().show_assignments();
        println!();
    }
}
