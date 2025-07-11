fn main() {
    // Declare a variable x in the outer scope
    let x = 5;
    println!("Outer x = {}", x); // prints 5

    {
        // Inner scope
        let x = x + 10; // Shadowing outer x
        println!("Inner x (shadowed) = {}", x); // prints 15

        let y = 20; // y exists only in this block
        println!("Inner y = {}", y); // prints 20
    }

    // y is out of scope here, so cannot be used
    // println!("Outer y = {}", y); // This would cause a compile error

    // Shadowing x again in the outer scope
    let x = x * 2;
    println!("Outer x after shadowing again = {}", x); // prints 10
}
