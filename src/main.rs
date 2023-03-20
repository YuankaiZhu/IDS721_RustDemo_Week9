use std::io;

// Linear Search
fn linear_search(arr: &[i32], target: i32) -> Option<usize> {
    for (i, &item) in arr.iter().enumerate() {
        if item == target {
            return Some(i);
        }
    }
    None
}

// Interpolation Search
fn interpolation_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() - 1;
    while low <= high && target >= arr[low] && target <= arr[high] {
        let pos = low + (((target - arr[low]) as f32 / (arr[high] - arr[low]) as f32) * (high - low) as f32) as usize;
        if arr[pos] == target {
            return Some(pos);
        } else if arr[pos] < target {
            low = pos + 1;
        } else {
            high = pos - 1;
        }
    }
    None
}

// Main function
fn main() {
    let mut arr_input = String::new();
    println!("Enter a list of integers separated by a space:");
    io::stdin().read_line(&mut arr_input).expect("Failed to read input");
    let arr: Vec<i32> = arr_input
        .trim()
        .split(' ')
        .map(|x| {
            x.parse().unwrap_or_else(|_| {
                eprintln!("Invalid input: {}", x);
                std::process::exit(1);
            })
        })
        .collect();
    println!("Enter the value to search for:");
    let mut target_input = String::new();
    io::stdin().read_line(&mut target_input).expect("Failed to read input");
    let target: i32 = match target_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid input: {}", target_input.trim());
            std::process::exit(1);
        }
    };
    println!("Choose a search algorithm:");
    println!("1. Linear search");
    println!("2. Interpolation search");
    let mut choice_input = String::new();
    io::stdin().read_line(&mut choice_input).expect("Failed to read input");
    let choice: i32 = match choice_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid input: {}", choice_input.trim());
            std::process::exit(1);
        }
    };
    let result = match choice {
        1 => linear_search(&arr, target),
        2 => interpolation_search(&arr, target),
        _ => {
            eprintln!("Invalid choice: {}", choice);
            std::process::exit(1);
        }
    };
    match result {
        Some(index) => println!("{} found at index {}", target, index),
        None => println!("{} not found in the array.", target),
    }
}
