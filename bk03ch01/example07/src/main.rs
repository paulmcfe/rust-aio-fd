trait Speak {
    fn speak(&self) -> String;
}

#[derive(Debug)]
struct Dog {
    name: String,
}

#[derive(Debug)]
struct Cat {
    name: String,
}

#[derive(Debug)]
struct Duck {
    name: String,
}

impl Speak for Dog {
    fn speak(&self) -> String {
        format!("{} says woof!", self.name)
    }
}

impl Speak for Cat {
    fn speak(&self) -> String {
        format!("{} says meow!", self.name)
    }
}

impl Speak for Duck {
    fn speak(&self) -> String {
        format!("{} says quack!", self.name)
    }
}
fn main() {
    let dog = Dog {
        name: "Chase".to_string(),
    };
    let cat = Cat {
        name: "Walter".to_string(),
    };
    let duck = Duck {
        name: "Daffy".to_string(),
    };

    println!("{}", dog.speak());
    println!("{}", cat.speak());
    println!("{}", duck.speak());
}
