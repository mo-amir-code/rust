use std::io;

struct Account {
    account_number: String,
    name: String,
    balance: u64
}

impl Account {
    fn check_balance(&self) -> u64 {
        return self.balance;
    }

    fn deposit(&mut self, amount: u64) {
        self.balance += amount;
        println!("{} amount added in your account", amount);
    }
}

fn main() {

    println!("Please enter choice: ");

    loop {
        println!("Create your account: ");
        println!("Check your balance: ");
        println!("Deposit money: ");
        println!("Withdrawal money: ");
    

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let selection = input.trim().parse().expect("Please enter a valid number");

        match selection {
            1 => {
                
            },
            _ => {

            }
        }

    }

}
