use core::fmt;

use colored::Colorize;
use serde::{Serialize, Deserialize};

use crate::cli::add::AddArgs;
use crate::utils::capitalize_first;

#[derive(Serialize, Deserialize)]
pub struct Assignment {
    name: String,
    weight: f32,
    count: usize,
    grades: Vec<f32>
}

impl From<&AddArgs> for Assignment {
    fn from(args: &AddArgs) -> Self {
        let grades = vec![];

        let mut asmt = Assignment {
            name: args.name().to_string(),
            weight: args.weight(),
            count: args.count(),
            grades
        };

        match asmt.add_grades(args.grades(), args.max(), false) {
            Ok(_) => {},
            Err(_) => {}
        };

        asmt
    }
}

impl PartialEq for Assignment {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq_ignore_ascii_case(&other.name)
    }
}

impl fmt::Display for Assignment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}: {:.2}",
            self.count.to_string().yellow(),
            capitalize_first(&self.name).magenta(),
            self.total()
        )
    }
}

impl fmt::Debug for Assignment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name_titled = capitalize_first(self.name());

        write!(f, "{}:\n", &name_titled)?;
        for i in 0..self.grades.len() {
            write!(
                f,
                "  {} {}: {}\n",
                &name_titled,
                i + 1,
                self.grades.get(i).unwrap()
            )?;
        }

        Ok(())
    }
}

impl Assignment {
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn total(&self) -> f32 {
        if self.count == 0 {
            return 0.0;
        }

        (&self.grades).into_iter().sum::<f32>() * self.weight / self.count as f32
    }

    pub fn add_grades(&mut self, grades: &Vec<f32>, out_of: f32, overwrite: bool) -> Result<(), ()> {
        if overwrite {
            self.grades.clear();
        }

        if self.grades.len() + grades.len() > self.count {
            return Err(());
        }

        for grade in grades {
            self.grades.push(*grade * 100.0 / out_of);
        }

        Ok(())
    }
}
