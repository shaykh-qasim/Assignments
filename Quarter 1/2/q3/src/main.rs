fn main() {

    let team:[&'static str;5] = ["Pakistan","India","Australia","England","Sri Lanka"];
        //                       0          1       2           3           4
    
    let win_year:[u32;9] = [1992,2011,1987,1999,2003,2007,2015,2019,1996];
        //                  0   1    2      3    4      5   6    7    8
    println!("Cricket Team: {}  -  Winning Year: {}\n",team[0],win_year[0]);
    println!("Cricket Team: {}  -  Winning Year: {}\n",team[1],win_year[1]);
    println!("Cricket Team: {}  -  Winning Year: {}, {}, {}, {}, {}\n",team[2],win_year[2],win_year[3],win_year[4],win_year[5],win_year[6]);
    println!("Cricket Team: {}  -  Winning Year: {}\n",team[3],win_year[7]);
    println!("Cricket Team: {}  -  Winning Year: {}\n",team[4],win_year[8]);
}
