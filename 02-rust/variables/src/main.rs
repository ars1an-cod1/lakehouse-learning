fn main() {
    let mut km = 45000;       // a variable is created with `let` // mut means this variable can be changeable next time
    println!("{}", km);
       // println! needs a format string; every value (number, text...) goes into a {}
    km=45200; // there is no need to write let again it needs only once before givin' variable
    println!("{}",km);

   let mut brand=("BMW ");
    println!("{}", brand );

    brand="AUDI";
    println!("{}", brand );
}