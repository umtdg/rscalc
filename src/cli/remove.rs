use clap::{Command, ArgMatches};

use crate::course::CourseList;

use super::root::course_list_arg;

pub fn remove_command() -> Command {
    Command::new("remove").alias("rm").alias("r")
        .about("Remove a course")
        .arg_required_else_help(true)
        .arg(course_list_arg())
}

pub fn remove_run(matches: &ArgMatches, course_list: &mut CourseList) {
    for course_name in matches.get_many::<String>("COURSES").unwrap() {
        course_list.0.remove(course_name);
    }
}
