struct Student {
    name: String,
    grade: i32,
}

impl Student {
    fn info(&self) {
        // is not able to edit just reads
        println!("{} has grade{}", self.name, self.grade); // info writes on terminal all the time
    }

    fn add_points(&mut self, points: i32) {
        self.grade += points;

        println!("{} has grade {} ", self.name, self.grade);
    }
}

fn main() {
    let mut arslan = Student {
        name: String::from("Arslan"),
        grade: 80,
    };

    arslan.info();
    arslan.add_points(10);
}
