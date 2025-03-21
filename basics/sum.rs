fn main() {
    println!("Hello, world!"); // Macro
    let a = 10;
    let b = 20;
    // let c = a + b; // Assigning a value to 'c'
    
    println!("\n\t Rust b: {1} & a: {0} \n\n\t c: a + b = {2}", a, b, a+b);
    let ans:u32=sum(126,129);
    println!("\n\t result : {}",ans);
    let op:i8=signedsum(-125,-3);
    println!("\n\t signed number : {}",op);
    let result=add(-5,15);
    print!("\n\t add : -5+15 :{} \n\n",result);
}

fn sum(a:u32,b:u32)->u32{
    return a+b;
}
fn signedsum(a:i8,b:i8)->i8{
    return a+b;
}
fn add(x:i8, y:i8)->i8{
    return x+y;
}
