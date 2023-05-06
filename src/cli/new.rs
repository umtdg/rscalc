use clap::{Command, ArgMatches};

use crate::course::{CourseList, Course};

use super::root::course_list_arg;

pub fn new_command() -> Command {
    Command::new("new").alias("n")
        .about("Create a new course")
        .arg_required_else_help(true)
        .arg(course_list_arg())
}

pub fn new_run(matches: &ArgMatches, course_list: &mut CourseList) {
    for course_name in matches.get_many::<String>("COURSES").unwrap() {
        if !course_list.0.contains_key(course_name) {
            course_list.0.insert(
                course_name.clone(),
                Course::new(course_name.clone())
            );
        }
    }
}
