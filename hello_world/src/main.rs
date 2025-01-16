fn main() {
    println!("Hello, world!"); 
    profile_function("Jaydip patel", 30, "1994-07-10", "male", 163.0, 62.5);
    let result = add_function(10, 2);
    println!("The result of the addition is {}", result);
    let result = subtract_function(112, 2);
    println!("The result of the subtraction is {}", result);
    let result = multiply_function(123, 2);
    println!("The result of the multiplication is {}", result);
    let result = divide_function(1234, 2);
    println!("The result of the division is {}", result);


    let mut _x: i32 = 10;
    let _y: &mut i32 = &mut _x; 
    *_y  += 1;
    *_y  -= 3;
    println!("{}", _x);


    let mut bank_account = BankAccount {
        owner: "Jaydip Patel".to_string(),
        balance: 1000.0,
    };  

    bank_account.withdraw(2300.0);
    bank_account.deposit(20309240.0);
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

fn profile_function(name: &str, age: u16, date_of_birth: &str, gender: &str, height: f32, weight: f32) {
    println!("****************************************************");
    println!("My name is {} and I am {} years old. I was born on {} and I am {} and my height is {} and my weight is {}", name, age, date_of_birth, gender, height, weight);
    println!("****************************************************");
}

fn add_function(a: i32, b: i32) -> i32 {
    a + b
}

fn subtract_function(a: i32, b: i32) -> i32 {
    a - b
}

fn multiply_function(a: i32, b: i32) -> i32 {
    a * b
}

fn divide_function(a: i32, b: i32) -> i32 {
    a / b
}

//ownership

 