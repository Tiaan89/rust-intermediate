struct BankAccount {
    balance: i32,
    verified: bool
}

// & takes the ownership away and allow borrowing
fn print_balance(account: &BankAccount) {
    println!("{:?}", account.balance);
}
fn print_verified(account: &BankAccount) {
    println!("{:?}", account.verified);
}

fn main() {
    let my_account = BankAccount {
        balance: 20,
        verified: true
    };

    print_balance(&my_account);
    print_verified(&my_account);

    //println!("{:?}", my_account.balance);
    //println!("{:?}", my_account.verified);
}
