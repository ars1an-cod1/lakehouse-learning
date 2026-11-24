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

fn is_adult(age: i32 ) -> bool{ // return type is gonna be boolean 

    return age>=18; // here is true or false
}

fn avarage(a:f64,b:f64,c:f64)->f64{

    return (a+b+c)/3.0;


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

    let new_is_adult = is_adult(21);
    println!("{}", new_is_adult);

    let new_avarage=avarage(23.0,56.0,100.0);
    println!("{}", new_avarage);
    
}