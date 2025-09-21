// The shared behavior
trait ProvidesEnergy {
    fn calories(&self) -> u32;
}

// The calories in an apple depend on its weight
struct Apple {
    weight_grams: f32,
}

// Implement ProvidesEnergy on Apple
impl ProvidesEnergy for Apple {
    fn calories(&self) -> u32 {
        // Apples have about 52 calories per 100g
        (self.weight_grams * 0.52) as u32
    }
}

// The calories in an energy bar are written on the package,
// so they don't require a calculation
struct EnergyBar {
    calories_per_bar: u32,
}

// Implement ProvidesEnergy on EnergyBar
impl ProvidesEnergy for EnergyBar {
    fn calories(&self) -> u32 {
        // Just return the calories
        self.calories_per_bar
    }
}

// This function is generic over T, 
// but constrains T to implement ProvidesEnergy
fn log_food_intake<T: ProvidesEnergy>(food: &T) {
    println!("Logging food with {} calories.", food.calories());
}

fn main() {
    let apple = Apple { weight_grams: 182.4 };

    let bar = EnergyBar { calories_per_bar: 255 };

    // You can pass both types to the same function!
    log_food_intake(&apple);
    log_food_intake(&bar);
}
