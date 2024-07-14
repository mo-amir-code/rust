use std::io;
use chrono::Utc;

use crate::lib::structs::{Account, TakeInputReturnOutput};

pub mod fns {
    use super::*;

    pub fn take_input(is_number: bool) -> TakeInputReturnOutput {
        let mut input_handle = String::new();
        io::stdin().read_line(&mut input_handle).expect("Failed to read line");
    
        if is_number {
            let num = input_handle.trim().parse().expect("Please enter a valid number");
            return TakeInputReturnOutput::Int(num);
        }
    
        TakeInputReturnOutput::Str(input_handle.trim().parse().expect("Please enter a valid number"))
    }
    
    pub fn return_str(val: &mut TakeInputReturnOutput) -> String {
        match val {
            TakeInputReturnOutput::Str(s) => {
                return s.to_string();
            },
            _ => {
                println!("Error: While returning string value");
            }
        }
    
        let s = String::from("None");
        s
    }
    
    pub fn return_int(val: &mut TakeInputReturnOutput) -> u64 {
        match val {
            TakeInputReturnOutput::Int(i) => {
                return *i;
            },
            _ => {
                println!("Error: While returning string value");
            }
        }
    
        1
    }
    
    pub fn create_unique_account_number() -> String {
        let timestamp = Utc::now().timestamp();
        let ac = format!("ABB{}",timestamp);
        ac
    }
    
    pub fn create_account(accounts: &mut Vec<Account>) {
    
        println!("Enter your name: ");
        let mut output: TakeInputReturnOutput = take_input(false);
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
    
    pub fn check_account_balance(accounts: &Vec<Account>) {
        println!("Enter your account number: ");
        let mut output: TakeInputReturnOutput = take_input(false);
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
    
            account.check_balance();
        }else{
            println!("Account not found");
        }
    }
    
    pub fn deposit(accounts: &mut Vec<Account>) {
        println!("Enter Account Number: ");
        let mut output: TakeInputReturnOutput = take_input(false);
        let account_number = return_str(&mut output);
    
    
        if let Some(account) = accounts.iter_mut().find(|acc| acc.account_number == account_number){
            println!("\nAccount found");
    
            println!("Enter Deposit Amount: ");
            output = take_input(true);
            let deposit_amount: u64 = return_int(&mut output);
            
            account.deposit(deposit_amount);
        }else{
            println!("Please provide correct account number to prcoeed next steps");
        }
       
    }
    
    pub fn withdrawal(accounts: &mut Vec<Account>){
        println!("Enter Account Number: ");
        let mut output: TakeInputReturnOutput = take_input(false);
        let account_number = return_str(&mut output);
    
        println!("Enter your account password: ");
        output = take_input(false);
        let password: String = return_str(&mut output);
    
    
        if let Some(account) = accounts.iter_mut().find(|acc| acc.account_number == account_number){
    
            if account.password != password {
                println!("Account number or password is wrong \n");
                return;
            }
    
            println!("\nAccount found");
    
            println!("Enter Withdraw Amount: ");
            output = take_input(true);
            let withdraw_amount: u64 = return_int(&mut output);
    
            account.withdrawal(withdraw_amount);
        }else{
            println!("Please provide correct account number to prcoeed next steps");
        }
    }
}