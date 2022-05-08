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

fn is_verified(account: &BankAccount) -> Result<bool, bool> {
    match account.verified {
        true => Ok(true),
        false => Err(false)
    }
}

fn main() {
    let my_account = BankAccount {
        balance: 20,
        verified: false
    };
    let verification_status = is_verified(&my_account)
        .unwrap(); //gets the value from a successful result

    let _verification_status_2 = is_verified(&my_account)
        .expect("Unable to unwrap results!");

    print_balance(&my_account);
    print_verified(&my_account);
    println!("{:?}", verification_status);

    //println!("{:?}", my_account.balance);
    //println!("{:?}", my_account.verified);
}
