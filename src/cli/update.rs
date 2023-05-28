use std::process::exit;

use clap::{ArgMatches, Command, Arg, arg, ArgAction};

use crate::{cli::root::common_args, course::CourseList};

pub struct UpdateArgs {
    course_name: String,
    name: String,
    overwrite: bool,
    remove: bool,
    max: f32,
    grades: Vec<f32>
}

impl UpdateArgs {
    pub fn name(&self) -> &str { &self.name }
    pub fn max(&self) -> f32 { self.max }
    pub fn overwrite(&self) -> bool { self.overwrite }
    pub fn remove(&self) -> bool { self.remove }
    pub fn grades(&self) -> &Vec<f32> { &self.grades }
}

impl From<&ArgMatches> for UpdateArgs {
    fn from(matches: &ArgMatches) -> Self {
        UpdateArgs {
            course_name: matches.get_one::<String>("course").expect("required").clone(),
            name: matches.get_one::<String>("name").expect("required").clone(),
            overwrite: matches.get_flag("overwrite"),
            remove: matches.get_flag("remove"),
            max: *matches.get_one::<f32>("max").unwrap(),
            grades: matches.get_many("GRADES").unwrap_or_default().copied().collect(),
        }
    }
}


pub fn update_command() -> Command {
    Command::new("update").alias("u")
        .about("Update an assignment")
        .arg_required_else_help(true)
        .args(update_args())
}

pub fn update_run(matches: &ArgMatches, course_list: &mut CourseList) {
    let update_args = UpdateArgs::from(matches);
    let course = course_list.find_course(&update_args.course_name);
    if course.is_none() {
        eprintln!("Course {} not found", update_args.course_name);
        exit(1);
    }

    let course = course.unwrap();
    let asmt = course.find_assignment(update_args.name());
    if asmt.is_none() {
        eprintln!(
            "Could not find assignment '{}' in '{}'",
            update_args.name(), course
        );
        exit(1);
    }

    if update_args.remove() {
        course.remove_assignment(update_args.name());
    } else if let Err(_) = asmt.unwrap().add_grades(
        update_args.grades(),
        update_args.max(),
        update_args.overwrite()
    ) {
        eprintln!("Number of grades exceeds maximum allowed");
    }
}

fn update_args() -> Vec<Arg> {
    let arg_overwrite = arg!(-o --overwrite <OVERWRITE> "Overwrite grades instead of appending")
        .action(ArgAction::SetTrue);

    let arg_rm = arg!(-r --remove <REMOVE> "Remove the assignment")
        .long("rm")
        .action(ArgAction::SetTrue);

    let mut args = common_args();

    args.push(arg_overwrite);
    args.push(arg_rm);

    args
}
