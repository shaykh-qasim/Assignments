use std::io;


fn main() {

            
            
               //height

                println!("Enter your height in cm : ");

            let mut height:std::string::String = String::new();     
                  io::stdin().read_line(&mut height)
                    .expect("Failed to read line");
        
            let mut heightcm:f64 = height.trim().parse()
                .expect("Please type a number!");

                

               //weight
    
            
            println!("Enter your weight in kg : ");
            let mut weight:std::string::String = String::new();
                  io::stdin().read_line(&mut weight)
                      .expect("Failed to read line");;

            let mut weightkg:f64 = weight.trim().parse()
                .expect("Please type a number!");    

            let heightm = heightcm / 100.0;

    
            //BMI CALCULATION

            let bmi = weightkg / (heightm*heightm);



        if bmi <= 18.5 {

        println!("\nYour BMI is {} and you are underweight.",bmi);
        }
        else if bmi > 25.0 && bmi < 18.5 {
            println!("\nYour BMI is {} and your weight is normal.",bmi);
        }
        else if bmi >= 25.0 {
            println!("\nYour BMI is {} and you are obese.",bmi);
        }

}
