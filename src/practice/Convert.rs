use std::io;

fn main(){
    println!("Enter a number :");
    let mut input:String=String ::new();
    io::stdin().read_line(&mut input).expect("Error ");

    let mut input:u32 =input.trim().parse().expect("Error");    
    println!("Received : {} ",input);

    let mut ans:String=String::new();
    
    while input>0 {
        let rem = input % 16;
        let newdec:String=match rem{
                0..=9 => rem.to_string(),
                10=>"A".to_string(),
                11=>"B".to_string(),
                12=>"C".to_string(),
                13=>"D".to_string(),
                14=>"E".to_string(),
                15=>"F".to_string(),
                _ => unreachable!()
            };
        ans.push_str(&newdec);// local=>ans
        input=input/16;
    };

                                                                            // ✅ safe manual reverse
                                                                            let mut chars: Vec<char> = ans.chars().collect();
                                                                            let mut low = 0;
                                                                            let mut high = chars.len() - 1;

                                                                            while low < high {
                                                                                chars.swap(low, high);
                                                                                low += 1;
                                                                                high -= 1;
                                                                            }
                                                                            let ans: String = chars.into_iter().collect();
                                                                            println!("Hexadecimal: {}", ans);

}
