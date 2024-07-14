mod lib;

use lib::functions::fns::{check_account_balance, create_account, deposit, take_input, withdrawal};
use lib::structs::{Account, TakeInputReturnOutput};


fn main() {

    let mut accounts: Vec<Account> = Vec::new();

    println!("\n\n <---- Welcome to ADLA BADLI Bank ----->  \n\n");

    

    loop {
        println!("1. Create your account: ");
        println!("2. Check your balance: ");
        println!("3. Deposit money: ");
        println!("4. Withdrawal money: ");
        println!("5. Exit: ");
        println!("\nEnter any number ===> ");


        let selection: TakeInputReturnOutput = take_input(true);

        match selection {
            TakeInputReturnOutput::Int(1) => {
                create_account(&mut accounts);
            },
            TakeInputReturnOutput::Int(2) => {
                check_account_balance(&accounts);
            },
            TakeInputReturnOutput::Int(3) => {
                deposit(&mut accounts);
            },
            TakeInputReturnOutput::Int(4) => {
                withdrawal(&mut accounts);
            },
            TakeInputReturnOutput::Int(5) => {
                break;
            },
            _ => {
                println!("Plese enter valid number \n");
            }
        }

    }

}

