use crate::line; // Import `line.rs`

pub fn triangle(n: u32) {
    for i in 1..=n {  // Corrected the range
        line::line(i);
    }
}
