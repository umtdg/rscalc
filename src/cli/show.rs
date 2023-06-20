use clap::{Command, arg, ArgAction, ArgGroup, ArgMatches};
use colored::Colorize;

use crate::course::{CourseList, Course};

use super::root::course_list_arg;

pub fn show_command() -> Command {
    Command::new("show").alias("s")
        .about("Show grades of courses")
        .arg(arg!(-g --grades <GRADES>... "Show grades of assignments")
            .action(ArgAction::SetTrue))
        .arg(arg!(-t --total <TOTAL> "Show only total grade")
            .action(ArgAction::SetTrue))
        .arg(arg!(-a --all <ALL> "Show all courses")
            .action(ArgAction::SetTrue))
        .arg(course_list_arg())
        .group(ArgGroup::new("COURSES_TO_SHOW")
            .args(["all", "COURSES"]))
}

pub fn show_run(matches: &ArgMatches, course_list: &mut CourseList) {
    let show_asmt_grades = matches.get_flag("grades");
    let show_total_only = matches.get_flag("total");

    let courses: Vec<&Course> = if matches.get_flag("all") {
        all_courses(course_list)
    } else {
        selected_courses(matches, course_list)
    };

    for course in courses {
        if show_total_only {
            println!(
                "{}: {}", course,
                format!("{:.2}", course.calculate_total()).green()
            );
        } else {
            course.show_assignments(show_asmt_grades);
            println!();
        }
    }
}

fn all_courses<'a>(course_list: &'a CourseList) -> Vec<&'a Course> {
    let mut sorted_courses: Vec<_> = (&course_list.0).into_iter().collect();
    sorted_courses.sort_by(|a, b| a.1.cmp(b.1));
    sorted_courses.iter().map(|(_, course)| *course).collect()
}

fn selected_courses<'a>(matches: &ArgMatches, course_list: &'a mut CourseList) -> Vec<&'a Course> {
    let mut courses: Vec<&Course> = Vec::new();

    // get all specified courses in matches from CourseList into courses
    for course_name in matches.get_many::<String>("COURSES").unwrap() {
        if let Some(course) = course_list.0.get(course_name) {
            courses.push(course);
        }
    }

    courses
}
