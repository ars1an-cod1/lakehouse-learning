fn car_status(km:i32){

    if km <= 20000{

        println!("{}","The car is new ");
    }
        else if km <= 100000{

            println!("{}", "This car is used");
        }
        else {

            println!("{}", "This car is old"); // when you dont have any variables there is no need to use {} because of it is place-holder like a f-string in python 
        }
}

fn day_name (day:i32){  // when there is no return we dont even need to return valuable 
    match day{

        1=> println!("Monday"), // in match option we use normal comma not a semi coolon
        2=> println!("Tuesday"),
        3=> println!("Wednesday"),
        4=> println!("Thursday"),
        5=> println!("Friday"),
        6=> println!("Saturday"),
        7=> println!("Sunday"), 
        _=> println!("Invalid"),//   _ if a wildcard: matches any value
    }



}




fn main(){

    car_status(23000);
    car_status(140000);
    car_status(5000);

    day_name(1);
    day_name(5);
    day_name(7);
    day_name(11);




}