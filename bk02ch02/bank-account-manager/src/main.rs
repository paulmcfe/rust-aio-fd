use std::io;

struct BankAccount {
    account_holder: String,
    balance: f64,
    account_number: String,
}

impl BankAccount {
    fn new(name: String, init_balance: f64, num: String) -> BankAccount {
        BankAccount {
            account_holder: name,
            balance: init_balance,
            account_number: num,
        }
    }

    fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
            println!("Successfully deposited ${:.2}", amount);
            println!("New balance: ${:.2}", self.balance);
        } else {
            println!("Deposit amount must be positive!");
        }
    }

    fn withdraw(&mut self, amount: f64) {
        if amount < 0.0 {
            println!("Withdrawal amount must be positive!");
        } else if amount > self.balance {
            println!("Insufficient funds! Your balance is ${:.2}", self.balance);
        } else {
            self.balance -= amount;
            println!("Successfully withdrew ${:.2}", amount);
            println!("Remaining balance: ${:.2}", self.balance);
        }
    }

    fn check_balance(&self) {
        println!("Current balance: ${:.2}", self.balance);
    }

    fn display_account_info(&self) {
        println!("\n--- Account Information ---");
        println!("Account Holder: {}", self.account_holder);
        println!("Account Number: {}", self.account_number);
        println!("Current Balance: ${:.2}", self.balance);
        println!("---------------------------");
    }
}

fn get_user_input() -> String {
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    input.trim().to_string()
}

fn get_amount() -> f64 {
    println!("Enter $ amount:");
    let input = get_user_input();
    input.parse::<f64>().unwrap_or_default()
}

fn main() {
    println!("Welcome to Simple Bank!");
    println!("=======================");

    // Create a new account
    println!("Let's set up your account.");

    println!("Enter your name:");
    let name = get_user_input();

    println!("Enter your desired account number:");
    let account_num = get_user_input();

    println!("Enter your initial deposit amount:");
    let initial_deposit = get_amount();

    let mut account = BankAccount::new(name, initial_deposit, account_num);

    println!("\nAccount created successfully!");
    account.display_account_info();

    // Main menu loop
    loop {
        println!("\nWhat would you like to do?");
        println!("1. Check Balance");
        println!("2. Make Deposit");
        println!("3. Make Withdrawal");
        println!("4. View Account Info");
        println!("5. Exit");
        println!("Enter your choice (1-5):");

        let choice = get_user_input();

        match choice.as_str() {
            "1" => {
                account.check_balance();
            }
            "2" => {
                println!("How much would you like to deposit?");
                let amount = get_amount();
                account.deposit(amount);
            }
            "3" => {
                println!("How much would you like to withdraw?");
                let amount = get_amount();
                account.withdraw(amount);
            }
            "4" => {
                account.display_account_info();
            }
            "5" => {
                println!("Thank you for using Simple Bank!");
                println!("Have a great day, {}!", account.account_holder);
                break;
            }
            _ => {
                println!("Invalid choice! Please enter 1, 2, 3, 4, or 5.");
            }
        }
    }
}
