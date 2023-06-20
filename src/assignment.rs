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

impl Eq for Assignment {
    fn assert_receiver_is_total_eq(&self) {}
}

impl PartialOrd for Assignment {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.grades.is_empty() == other.grades.is_empty() {
            self.weight.partial_cmp(&other.weight)
        } else if self.grades.is_empty() {
            Some(std::cmp::Ordering::Greater)
        } else if other.grades.is_empty() {
            Some(std::cmp::Ordering::Less)
        } else { // Never runs
            self.weight.partial_cmp(&other.weight)
        }
    }
}

impl Ord for Assignment {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.grades.is_empty() == other.grades.is_empty() {
            self.weight.total_cmp(&other.weight)
        } else if self.grades.is_empty() {
            std::cmp::Ordering::Greater
        } else if other.grades.is_empty() {
            std::cmp::Ordering::Less
        } else { // Never runs
            self.weight.total_cmp(&other.weight)
        }
    }
}

impl fmt::Display for Assignment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} ({}): {}",
            self.count.to_string().yellow(),
            capitalize_first(&self.name).magenta(),
            format!("{:.2}", self.weight * 100.0).green(),
            format!("{:.2}", self.total()).cyan()
        )
    }
}

impl fmt::Debug for Assignment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name_capitalized = capitalize_first(self.name());

        write!(
            f, "{} ({}%):\n",
            &name_capitalized.magenta().bold(),
            format!("{:.2}", self.weight * 100.0).green()
        )?;
        for (i, grade) in self.grades.iter().enumerate() {
            write!(
                f, "    {}: {} ({})\n",
                format!("{} {}", &name_capitalized, i + 1).yellow(),
                format!("{:.2}", grade).green(),
                format!("{:.2}", grade * self.weight / self.count as f32).cyan()
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
