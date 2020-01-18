fn main() {
    let data:(&'static str, u32, f64) = ("Banana", 120, 23.6);
    let (fruit, weight, price) = data;

    println!("Fruit: {}\n",fruit);
println!("Weight: {} (grams)\n",weight);
println!("Price/kg :{}\n",price);
println!("\nComplete Tuple: {:?}",data);


}
