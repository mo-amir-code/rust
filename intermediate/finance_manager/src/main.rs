use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};

#[derive(Serialize, Deserialize, Debug)]
enum TransactionType {
    Income,
    Expense,
}

#[derive(Serialize, Deserialize, Debug)]
struct Transaction {
    t_type: TransactionType,
    amount: f64,
    description: String,
}

impl Transaction {
    fn new(t_type: TransactionType, amount: f64, description: String) -> Transaction {
        Transaction {
            t_type,
            amount,
            description,
        }
    }
}

const FILE_PATH: &str = "transactions.json";

fn add_transaction(transactions: &mut Vec<Transaction>, t_type: TransactionType, amount: f64, description: String) {
    transactions.push(Transaction::new(t_type, amount, description));
}

fn save_transactions(transactions: &Vec<Transaction>) -> io::Result<()> {
    let data = serde_json::to_string(transactions)?;
    fs::write(FILE_PATH, data)?;
    Ok(())
}

fn load_transactions() -> io::Result<Vec<Transaction>> {
    let data = fs::read_to_string(FILE_PATH)?;
    let transactions: Vec<Transaction> = serde_json::from_str(&data)?;

    Ok(transactions)
}

fn main() {
    let mut transactions = load_transactions().unwrap_or_else(|_| Vec::new());

    loop {
        println!("\n1. Add Income");
        println!("2. Add Expense");
        println!("3. View Transactions");
        println!("4. Generate Report");
        println!("5. Save and Exit");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "1" => {
                add_transaction_prompt(&mut transactions, TransactionType::Income);
            }
            "2" => {
                add_transaction_prompt(&mut transactions, TransactionType::Expense);
            }
            "3" => {
                view_transactions(&transactions);
            }
            "4" => {
                generate_report(&transactions);
            }
            "5" => {
                save_transactions(&transactions).expect("Failed to save transactions");
                break;
            }
            _ => eprintln!("Invalid choice"),
        }
    }
}

fn add_transaction_prompt(transactions: &mut Vec<Transaction>, t_type: TransactionType) {
    print!("Enter amount: ");
    io::stdout().flush().unwrap();
    let mut amount_str = String::new();
    io::stdin().read_line(&mut amount_str).expect("Failed to read line");
    let amount: f64 = amount_str.trim().parse().expect("Invalid amount");

    print!("Enter description: ");
    io::stdout().flush().unwrap();
    let mut description = String::new();
    io::stdin().read_line(&mut description).expect("Failed to read line");

    add_transaction(transactions, t_type, amount, description.trim().to_string());
}

fn view_transactions(transactions: &Vec<Transaction>) {
    for (i, transaction) in transactions.iter().enumerate() {
        let t_type = match transaction.t_type {
            TransactionType::Income => "Income",
            TransactionType::Expense => "Expense",
        };
        println!("{}: {} - ${:.2} - {}", i + 1, t_type, transaction.amount, transaction.description);
    }
}

fn generate_report(transactions: &Vec<Transaction>) {
    let total_income: f64 = transactions.iter()
        .filter(|t| matches!(t.t_type, TransactionType::Income))
        .map(|t| t.amount)
        .sum();

    let total_expense: f64 = transactions.iter()
        .filter(|t| matches!(t.t_type, TransactionType::Expense))
        .map(|t| t.amount)
        .sum();

    let balance = total_income - total_expense;

    println!("\nFinancial Report:");
    println!("Total Income: ${:.2}", total_income);
    println!("Total Expense: ${:.2}", total_expense);
    println!("Balance: ${:.2}", balance);
}
