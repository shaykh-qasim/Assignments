fn main() {
    let number = 5;
    let receive = square(number);
    println!("The square is : {:?}",receive);

}

fn square(val: i64) -> (i64) {
    println!("The number is : {}",val);
    (val * val)
}