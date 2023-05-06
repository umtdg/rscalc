use std::process::exit;

use clap::{ArgMatches, Command, Arg, arg, value_parser};

use crate::{cli::root::common_args, course::CourseList, assignment::Assignment};

pub struct AddArgs {
    course_name: String,
    name: String,
    weight: f32,
    count: usize,
    max: f32,
    grades: Vec<f32>
}

impl AddArgs {
    pub fn name(&self) -> &str { &self.name }
    pub fn weight(&self) -> f32 { self.weight }
    pub fn count(&self) -> usize { self.count }
    pub fn max(&self) -> f32 { self.max }
    pub fn grades(&self) -> &Vec<f32> { &self.grades }
}

impl From<&ArgMatches> for AddArgs {
    fn from(matches: &ArgMatches) -> Self {
        AddArgs {
            course_name: matches.get_one::<String>("course").expect("required").clone(),
            name: matches.get_one::<String>("name").expect("required").clone(),
            weight: *matches.get_one::<f32>("weight").unwrap_or(&0.0) / 100.0,
            count: *matches.get_one::<usize>("count").unwrap_or(&0),
            max: *matches.get_one::<f32>("max").unwrap(),
            grades: matches.get_many("GRADES").unwrap_or_default().copied().collect(),
        }
    }
}

pub fn add_command() -> Command {
    Command::new("add").alias("a")
        .about("Add an assignment to a course")
        .arg_required_else_help(true)
        .args(add_args())
}

pub fn add_run(matches: &ArgMatches, course_list: &mut CourseList) {
    let add_args = AddArgs::from(matches);
    let course = course_list.find_course(&add_args.course_name);
    if course.is_none() {
        eprintln!("Course {} not found", add_args.course_name);
        exit(1);
    }

    (*course.unwrap()).add_assignment(Assignment::from(&add_args));
}

fn add_args() -> Vec<Arg> {
    let arg_weight = arg!(-w --weight <WEIGHT> "Total weight of the assignment (as decimal)")
        .value_parser(value_parser!(f32))
        .required(true);

    let arg_count = arg!(-C --count <COUNT> "Number of assignments")
        .value_parser(value_parser!(usize))
        .required(true);

    let mut args = common_args();

    args.push(arg_weight);
    args.push(arg_count);

    args
}
