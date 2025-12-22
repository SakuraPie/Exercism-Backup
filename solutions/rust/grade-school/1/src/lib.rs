use std::collections::HashMap;

pub struct School {
    roster: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            roster: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        for kv in &self.roster {
            for s in kv.1 {
                if student == *s {
                    return;
                }
            }
        }
        if let Some(v) = self.roster.get(&grade) {
            self.roster.insert(grade, v.clone());
        } else {
            self.roster.insert(grade, vec![]);
        }
        self.roster
            .get_mut(&grade)
            .unwrap()
            .push(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades = vec![];
        for kv in &self.roster {
            grades.push(*kv.0);
        }
        grades.sort();
        grades
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        if let Some(v) = self.roster.get(&grade) {
            let mut mv = v.clone();
            mv.sort();
            mv
        } else {
            vec![]
        }
    }
}