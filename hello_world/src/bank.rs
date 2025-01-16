
fn main() {
    let mut bank_account = BankAccount {
        owner: "Jaydip Patel".to_string(),
        balance: 1200.0,
    };  

    bank_account.withdraw(4025.0);
    bank_account.deposit(230.0);
    println!("Current balance: {}", bank_account.get_balance());
}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!("Withdrawing {} from {}", amount, self.owner);
        if self.balance >= amount {
            self.balance -= amount;
            println!("Withdrawal successful. New balance: {}", self.balance);
        } else {
            println!("Insufficient funds");
        }
    }

    fn deposit(&mut self, amount: f64) {
        println!("Depositing {} to {}", amount, self.owner);
        self.balance += amount;
        println!("Deposit successful. New balance: {}", self.balance);
    }

    fn get_balance(&self) -> f64 {
        println!("Getting balance for {}", self.owner);
        self.balance
    }
}