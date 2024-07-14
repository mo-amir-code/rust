

#[derive(Debug)]
pub struct Account {
    pub account_number: String,
    pub balance: u64,
    pub name: String,
    pub password: String
}

pub enum TakeInputReturnOutput {
    Str(String),
    Int(u64)
}

impl Account {
    pub fn check_balance(&self) {
        println!("Name: {}", self.name);
        println!("Balance: {}", self.balance);
        println!("Account Number: {} \n\n", self.account_number);
    }

    pub fn deposit(&mut self, amount: u64) {
        self.balance += amount;
        println!("{} amount added in your account", amount);
    }

    pub fn withdrawal(&mut self, amount: u64) {
        if !(self.balance >= amount) {
            println!("Insufficient balance");
            return;
        }

        self.balance -= amount;
        println!("{} amount withdrawaled from your account", amount);
    }
}