use clap::Command;

pub fn list_command() -> Command {
    Command::new("list").alias("ls").alias("l")
        .about("List courses")
}