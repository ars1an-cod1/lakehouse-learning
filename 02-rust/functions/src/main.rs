fn add_km(km: i32 , distance: i32)->i32{
   return km + distance; // there is no ; because it returns 
}
fn double(number: i32)->i32{
    return number*2;
}

fn add(number1:i32 ,number2:i32)->i32{
    return number1+number2;
}

fn greet(name: &str){
    println!("{}",name);
}

fn discount(price:f64, percent:f64)->f64{
    return price- price*percent/100.0;
}

fn main(){
    let new_km=add_km(45200,200);
    println!("{}", new_km);

    let new_number=double(30);
    println!("{}",new_number);

    let sum_number=add(12,13);
    println!("{}", sum_number);

    greet("Arslan"); // it returned from println! in function if it wouldnt be there we couldnt print "Arslan"

    let discount_price=discount(1400.0,15.0);
    println!("{}{}", "New price with discount :",discount_price);
}