fn main() {
    let r;
    {
        // Start of a new scope.
        let x = 5; // x is local to this scope.
        r = &x; // Initialize r to be a reference to x.
    } // The scope ends here, so x goes away. Uh oh.
    println!("{}", r); // r is now pointing to freed memory. Not good!
}
