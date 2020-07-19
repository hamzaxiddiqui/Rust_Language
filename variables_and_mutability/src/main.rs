
// fn main() { // this program will prompt error because of immmutability issue
//     let x =6;
//     println!("Value of x is {}",x);
//     x=8;
//     println!("Value of x is {}",x);
// }

// fn main (){ // this function is the example of mutable variable by using 'mut' keyword
//     let mut x =6;
//     println!("Value of x is {}",x);
//     x=80;
//     println!("Value of x is {}",x);
// }

fn main(){ //this function is the example of constant variable by using 'const' keyword
    const Y:u32 = 52;
    println!("Value of x is {}",Y);
}