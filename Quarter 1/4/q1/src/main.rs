fn main() {
    let number = integer(256);
    println!("The number is : {}\n",number);

    if number == 0 {
        println!("The number is equal to 0 \n");
    }
    else if number > 0 {
        println!("The number is positive \n");
    }
    else {
        println!("The number is negative \n");
    }

}

fn integer(parameter: i64) -> i64 {
    parameter
}