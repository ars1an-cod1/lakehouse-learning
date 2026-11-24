struct Car {
    brand: String,
    km: i32, // fields and their  types
    is_sold: bool,
}

impl Car {
    // the methods of Car live here

    fn drive(&mut self, distance: i32) {
        // &= borrow mut=edit self=this car

        self.km = self.km + distance;
        println!("{} now has {} km", self.brand, self.km);
    }
}

fn main() {
    let mut bmw = Car {
        brand: String::from("BMW "),
        km: 45000,
        is_sold: false,
    };
    bmw.drive(200);
    bmw.drive(50);
}
