use std::io;
use chrono::Utc;

#[derive(Debug)]
struct Account {
    account_number: String,
    balance: u64,
    name: String,
    password: String
}

enum take_input_return_output {
    Str(String),
    Int(i64)
}

impl take_input_return_output {
    fn as_str(&self) -> Option<&str> {
        if let take_input_return_output::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }

    fn as_int(&self) -> Option<i64> {
        if let take_input_return_output::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }
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

    let mut accounts: Vec<Account> = Vec::new();

    println!("\n\n <---- Welcome to ADLA BADLI Bank ----->  \n\n");

    

    loop {
        println!("1. Create your account: ");
        println!("2. Check your balance: ");
        println!("3. Deposit money: ");
        println!("4. Withdrawal money: ");
        println!("\nEnter any number ===> ");


        let selection: take_input_return_output = take_input(true);

        match selection {
            take_input_return_output::Int(1) => {
                create_account(&mut accounts);
            },
            take_input_return_output::Int(2) => {
                check_account_balance(&accounts);
            },
            _ => {

            }
        }

    }

}

fn take_input(is_number: bool) -> take_input_return_output {
    let mut input_handle = String::new();
    io::stdin().read_line(&mut input_handle).expect("Failed to read line");

    if is_number {
        let num = input_handle.trim().parse().expect("Please enter a valid number");
        return take_input_return_output::Int(num);
    }

    take_input_return_output::Str(input_handle.trim().parse().expect("Please enter a valid number"))
}

fn return_str(val: &mut take_input_return_output) -> String {
    match val {
        take_input_return_output::Str(s) => {
            return s.to_string();
        },
        _ => {
            println!("Error: While returning string value");
        }
    }

    let s = String::from("None");
    s
}

fn return_int(val: &mut take_input_return_output) -> i64 {
    match val {
        take_input_return_output::Int(i) => {
            return 1;
        },
        _ => {
            println!("Error: While returning string value");
        }
    }

    -1
}

fn create_unique_account_number() -> String {
    let timestamp = Utc::now().timestamp();
    let ac = format!("ABB{}",timestamp);
    ac
}

fn create_account(accounts: &mut Vec<Account>) {

    println!("Enter your name: ");
    let mut output: take_input_return_output = take_input(false);
    let name: String = return_str(&mut output);

    println!("Enter your bank account password: ");
    output = take_input(false);
    let password: String = return_str(&mut output);

    let account_number: String = create_unique_account_number();

    let new_user_account: Account = Account{
        account_number,
        balance: 0,
        name,
        password
    };

    println!("\n {:?}", new_user_account);
    accounts.push(new_user_account);

    println!("Your account created successfully...! \n\n");
}

fn check_account_balance(accounts: &Vec<Account>) {
    println!("Enter your account number: ");
    let mut output: take_input_return_output = take_input(false);
    let account_number: String = return_str(&mut output);

    println!("Enter your account password: ");
    output = take_input(false);
    let password: String = return_str(&mut output);

    if let Some(account) = accounts.iter().find(|acc| acc.account_number == account_number) {

        if account.password != password {
            println!("Account number or password is wrong \n");
            return;
        }

        println!("\nAccount found: ");
        println!("Name: {}", account.name);
        println!("Balance: {}", account.balance);
        println!("Account Number: {} \n\n", account.account_number);
    }else{
        println!("Account not found");
    }
}