// fn main() {
//     let mut _x: i32 = 5;
//     let _r: &mut i32 = &mut _x;
//     *_r += 1;
//     *_r -= 3;
//     println!("The value of x is {}", _x);
//     // println!("The value of r is {}", _r);

// }

// fn main() {
//     let mut account: BankAccount = BankAccount{
//         owner: "Alice".to_string(),
//         balance: 1050.43,
//     };

//     account.check_balance();
//     account.withdraw(500.2);
//     account.check_balance();
// }

// struct BankAccount {
//     owner: String,
//     balance: f64,
// }

// impl BankAccount {
//     fn withdraw(&mut self, amount: f64) {
//         println!("Withdraewing {} from account owned by {}", amount, self.owner );
//         self.balance -= amount;
//     }

//     fn check_balance(&self) {
//         println!("Account owned by {} has a balance of {}", self.owner, self.balance)
//     }
// }

fn main() {
    let mut account: BankAccount = BankAccount{
        owner: "Trevor".to_string(),
        balance: 100000.00
    };

    account.check_cash();
    account.withdraw_cash(50000.00);
    account.check_cash();
    account.add_cash(200000.00);
    account.check_cash();
}

struct BankAccount {
    owner: String,
    balance: f64
}

impl BankAccount {

    fn withdraw_cash(&mut self, amount: f64) {
        println!("Your withdrawing {:.2} from {}'s Account", amount, self.owner);
        self.balance -= amount;
    }

    fn add_cash(&mut self, amount: f64) {
        println!("Your adding {:.2} to {}'s account", amount, self.owner);
        self.balance += amount;
    }

    fn check_cash(&mut self) {
        println!("Currently {} has {:.2} in their account", self.owner, self.balance);
    }

}