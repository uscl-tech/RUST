mod line; // Import the line module
mod tri;  // Import the triangle module

fn main() {
    for i in 0..10 {
        println!("{}", i);
    }
    println!("\n\t Counter : {}", counter(9));
    println!("\n\t Line \n");
    line::line(5);
    println!("\n\t Triangle \n");
    tri::triangle(5);
}

fn counter(n: u32) -> u32 {
    n + 1  // Removed unnecessary return statement
}






0
1
2
3
4
5
6
7
8
9

         Counter : 10

         Line 

        *       *       *       *       *


         Triangle 

        *

        *       *

        *       *       *

        *       *       *       *

        *       *       *       *       *

vikram@vikram-inspiron3584:~/Rust/RUST/loops$ 









