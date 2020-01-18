fn main() {
    let integer1 = 55;
    let integer2:f64 = 32.5;

    let sum = integer1 as f64 + integer2;
    let sub = integer1 as f64 - integer2;
    let mul = integer1 as f64 * integer2;
    let div = integer1 as f64 / integer2;

    println!("The sum of {} and {} is  {} \n",integer1,integer2,sum);
    println!("Subtracting {} from {} gives {}\n",integer2,integer1,sub);
    println!("Multiplying {} with {} gives {}\n",integer1,integer2,mul);
    println!("Dividing {} from {} gives {}\n",integer2,integer1,div);

}