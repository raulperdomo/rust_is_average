use std::io;
use std::collections::HashMap;

fn main() {
    println!("Welcome to Measures of Central Tendancy!");

    let mut vec = Vec::new();

    add_to_vector(&mut vec);
    
    //dbg!(&vec);
    //vec.sort();
    //dbg!(&vec);
    println!("Current vector: {:?}", vec);

    loop {
        println!("Which measure of central tendancy are you interested in?");
        println!("(1) Median");
        println!("(2) Mode");
        println!("(3) Mean");
        println!("(4) Add more integers to vector");
        println!("(q) to Quit");


        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read choice.");
        match choice.trim() {
            "1" => median(&mut vec),
            "2" => mode(&mut vec),
            "3" => mean(&mut vec),
            "4" => { 
                add_to_vector(&mut vec);
                println!("Current vector: {:?}", vec);
            },
            "q" => { println!("Good-Bye!"); break }
            _ => { println!("Invalid entry.");}
            
        };
        println!("")
    }
}

fn add_to_vector(vec: &mut Vec<i32>) {
    println!("Please enter a value or hit enter to calculate the averages.");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line.");

        let input: i32 = match input.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                println!("No text entered, breaking input loop.");
                break;
            }
        };
        vec.push(input);
    }
}

fn median(vec: &mut Vec<i32>){
    vec.sort();
    //dbg!(vec);
    if vec.len() % 2 == 0 {
        let low_index = vec.len() / 2 - 1;
        let high_index = low_index + 1;
        let median = (vec[low_index] as f32 + vec[high_index] as f32) / 2.0;
        //dbg!(&vec, low_index, high_index, median);
        println!("The median is: {median}")
    }
    else {
        let middle = (vec.len() / 2) + 1;
        let median = vec[middle];
        println!("The median is: {median}")
    }
}

fn mode(vec: &mut Vec<i32>) {
    vec.sort();
    let mut occurrences: HashMap<i32, i32> = HashMap::new(); 
    for i in vec.iter() {
        let _ = *occurrences.entry(*i)
        .and_modify(|counter| { *counter += 1})
        .or_insert(1);
    }
    let mut mode = 0;
    let mut count = 0;
    for (k, v) in occurrences.iter() {
        //dbg!(&mode, k, v);
        if *v > count {
            mode = *k;
            count = *v;
        }
    }
    println!("The mode is: {mode}")
    
}

fn mean(vec: &mut Vec<i32>) {
    let mut sum: i32 = 0;
    for i in vec.iter() {
        sum += *i;
    }
    let mean = sum as f32 / vec.len() as f32;
    println!("The mean is {mean}");
}
