fn main() {
    let g = String::from("hello world");
    println!("\n\t {}", g);

    // Pass 'g' to the function
    reverse_string(&g);
}

// Function to reverse a given string
fn reverse_string(input: &str) {
    let result: String = input.chars().rev().collect();

    println!("\n\t Original : {}", input);
    println!("\n\t Reversed : {}", result);
}

