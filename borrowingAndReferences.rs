// fn main() {
//     let mut _x: i32 = 5;
//     let _r: &mut i32 = &mut _x;
//     *_r += 1;
//     *_r -= 3;
//     println!("The value of x is {}", _x);
//     // println!("The value of r is {}", _r);

// }

fn main() {
    let mut account: BankAccount = BankAccount{
        owner: "Alice".to_string(),
        balance: 1050.43,
    };

    account.check_balance();
    account.withdraw(500.2);
    account.check_balance();
}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!("Withdraewing {} from account owned by {}", amount, self.owner );
        self.balance -= amount;
    }

    fn check_balance(&self) {
        println!("Account owned by {} has a balance of {}", self.owner, self.balance)
    }
}