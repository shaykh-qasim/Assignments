use std::io ;

fn main() {

    let name:String ;
    let marks1:u64 ;
    let marks2:u64 ;

    let mut input = String::new();

    //.................................
    println!("\nEnter name: ");
    io::stdin().read_line(&mut input).expect("Failed!");
    name = input;
    input = "".to_string();
    println!("{}[2J", 27 as char);    //CLEARING TERMINAL

    //.................................
    println!("\nEnter Obtained Marks in Subject 1: ");
    io::stdin().read_line(&mut input).expect("Failed!");
    marks1 = input.trim().parse().expect("Failed!");
    input = "".to_string();
    println!("{}[2J", 27 as char);    //CLEARING TERMINAL


    //.................................
    println!("\nEnter Obtained Marks in Subject 2: ");
    io::stdin().read_line(&mut input).expect("Failed!");
    marks2 = input.trim().parse().expect("Failed!");
    input = "".to_string();
    println!("{}[2J", 27 as char);    //CLEARING TERMINAL



    let percent = data_calculation(name, marks1, marks2);       //sending data to data_calculation function

    
    
    if percent > 100 as f64 {
        println!("{}[2J", 27 as char);    //CLEARING TERMINAL TO EMPHASIZE THE ERROR
        println!("\nENTER VALID MARKS UPTO 100.");
    }
    
    else if percent > 70 as f64 {
        println!("\nPASS WITH {}%",percent);
    }
    else {
        println!("\nYOU ARE FAILED. YOUR PERCENTAGE IS : {}%", percent);
    }


}



fn data_calculation(name: String, sub1: u64, sub2: u64) -> f64 {
    
    let percentage = ((sub1 as f64 + sub2 as f64) / 200 as f64) * 100 as f64 ;
        

        println!("NAME : {}",name);
        println!("MARKS SUBJECT 1 : {}/100",sub1);
        println!("MARKS SUBJECT 2 : {}/100",sub2);

    percentage
    

}