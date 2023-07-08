//Basic Macro Practice

macro_rules! say_jimston {
    () => {
        println!("Jimston Dylan!")
    };
}

macro_rules! say_jumanji {
    () => {
        println!("kodak man bisa lompat")
    };
}

fn main() {
    let mut mutable_value: i8 = 0;
    mutable_value += 1;
    match mutable_value {
        1 => say_jimston!(),
        _ => println!("Darn Diddly Seung Jae Lee")        
    };
}
