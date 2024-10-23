mod bank_account;

use crate::bank_account::BankAccount;

fn main(){
    let account = BankAccount::new(500.0);

    println!("account: {:?}", account);
}
