use clap::{Arg, Command, arg, value_parser};

use super::add::add_command;
use super::list::list_command;
use super::new::new_command;
use super::remove::remove_command;
use super::show::show_command;
use super::update::update_command;


// TODO: Refactor each subcommand into its own module
pub fn root_command() -> Command {
    Command::new("rscalc")
        .about("Grades calculator")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .arg(arg!(-f --file <FILE> "Grades file")
            .required(false))
        .subcommand(list_command())
        .subcommand(new_command())
        .subcommand(add_command())
        .subcommand(update_command())
        .subcommand(show_command())
        .subcommand(remove_command())
}

pub fn course_arg() -> Arg {
    arg!(-c --course <COURSE> "Course")
        .required(true).num_args(1)
}

pub fn course_list_arg() -> Arg {
    arg!(<COURSES> "Course list").num_args(1..)
}

pub fn common_args() -> Vec<Arg> {
    let arg_name = arg!(-n --name <NAME> "Assignment name")
        .required(true);

    let arg_grades = arg!(<GRADES> "Assignment grades")
        .value_parser(value_parser!(f32))
        .required(false)
        .num_args(0..);

    let arg_max = arg!(-m --max <MAX> "Maximum grade for a single assignment")
        .value_parser(value_parser!(f32))
        .required(false)
        .default_value("100.0")
        .default_missing_value("100.0");

    vec![course_arg(), arg_name, arg_grades, arg_max]
}
