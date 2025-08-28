struct BankAccount {
    balance: f64,
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64) -> Result<f64, String> {
        if amount <= 0.0 {
            Err(String::from("Withdrawal amount must be positive."))
        } else if amount > self.balance {
            Err(String::from("Insufficient funds"))
        } else {
            self.balance -= amount;
            Ok(amount)
        }
    }

    fn get_balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
    let mut account = BankAccount { balance: 100.0 };
    println!("Initial balance: ${:.2}", account.get_balance());

    match account.withdraw(200.0) {
        Ok(amount) => println!("Withdrew: ${:.2}", amount),
        Err(error) => println!("Withdrawal failed: {}", error),
    }

    match account.withdraw(50.0) {
        Ok(amount) => println!("Withdrew: ${:.2}", amount),
        Err(error) => println!("Withdrawal failed: {}", error),
    }
}
