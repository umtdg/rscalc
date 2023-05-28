use std::collections::HashMap;
use std::fs::OpenOptions;
use std::fmt;

use colored::Colorize;
use serde::{Serialize, Deserialize};

use crate::assignment::Assignment;

#[derive(Serialize, Deserialize)]
pub struct Course {
    name: String,
    assignments: HashMap<String, Assignment>
}

impl PartialEq for Course {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq_ignore_ascii_case(&other.name)
    }
}

impl Eq for Course {
    fn assert_receiver_is_total_eq(&self) {}
}

impl fmt::Display for Course {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name.to_uppercase().bold().blue())
    }
}

impl Course {
    pub fn new(name: String) -> Course {
        let assignments = HashMap::new();
        Course {
            name,
            assignments
        }
    }

    pub fn add_assignment(&mut self, asmt: Assignment) {
        self.assignments.insert(asmt.name().to_string(), asmt);
    }

    pub fn remove_assignment(&mut self, name: &str) -> Option<Assignment> {
        self.assignments.remove(name)
    }

    pub fn find_assignment(&mut self, name: &str) -> Option<&mut Assignment> {
        self.assignments.get_mut(name)
    }

    pub fn calculate_total(&self) -> f32 {
        (&self.assignments).into_iter().map(|(_, asmt)| asmt.total()).sum()
    }

    pub fn show_assignments(&self, show_asmt_grades: bool) {
        println!("{}", self);

        for asmt in (&self.assignments).values() {
            if show_asmt_grades {
                println!("  {:?}", asmt);
            } else {
                println!("  {}", asmt);
            }
        }

        println!(
            "  {}: {:.2}",
            "Total".bold().green(),
            self.calculate_total()
        );
    }
}

pub enum SaveError {
    Io(std::io::Error),
    Serde(serde_json::Error)
}

impl From<std::io::Error> for SaveError {
    fn from(err: std::io::Error) -> Self {
        SaveError::Io(err)
    }
}

impl From<serde_json::Error> for SaveError {
    fn from(err: serde_json::Error) -> Self {
        SaveError::Serde(err)
    }
}

impl fmt::Display for SaveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(err) => write!(f, "IO Error: {}", err),
            Self::Serde(err) => write!(f, "JSON Error: {}", err)
        }
    }
}


#[derive(Serialize, Deserialize)]
pub struct CourseList(pub HashMap<String, Course>);

impl fmt::Display for CourseList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Courses:\n")?;

        for course in self.0.values() {
            write!(f, "  {}\n", course)?;
        }

        Ok(())
    }
}

impl CourseList {
    pub fn new() -> Self {
        CourseList(HashMap::new())
    }

    pub fn load(file_path: &str) -> Self {
        let file = OpenOptions::new()
            .read(true)
            .open(file_path);

        if file.is_err() {
            eprintln!("Course file not found: '{}'. Loading empty course list", file_path);
            return CourseList::new();
        }

        serde_json::from_reader(&file.unwrap()).unwrap_or(CourseList::new())
    }

    pub fn save(&self, file_path: &str) -> Result<(), SaveError> {
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(file_path)?;

        serde_json::to_writer_pretty(&file, &self.0)?;

        Ok(())
    }

    pub fn find_course(&mut self, course_name: &str) -> Option<&mut Course> {
        self.0.get_mut(course_name)
    }
}
