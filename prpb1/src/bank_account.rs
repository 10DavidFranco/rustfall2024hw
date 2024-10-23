#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        // Implement this method
        BankAccount{
        balance: initial_balance,
        }
    }

    pub fn deposit(&mut self, amount: f64) {
        // Implement this method
        if amount < 0.0{
            println!("Attempting to deposit negative amount, balance not changed")
            
        }else{


        self.balance += amount;
        }
    }

    pub fn withdraw(&mut self, amount: f64) {
        // Implement this method
       


        self.balance -= amount;

       if self.balance < 0.0{
           println!("Account overdrawn");
           self.balance = 0.0;
       }
    }

    pub fn balance(&self) -> f64 {
        // Implement this method
        return self.balance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_account() {
        // Write a test for creating a new account

        let account = BankAccount::new(500.0);
        assert_eq!(account.balance(),500.0);
    }

    
    #[test]
    fn test_deposit() {
        // Write a test for depositing money
        let mut account = BankAccount::new(0.0);
        account.deposit(500.0);
        assert_eq!(account.balance(), 500.0);
        account.deposit(-100.0);
        assert_eq!(account.balance(), 500.0);
    }

 
    #[test]
    fn test_withdraw() {
        // Write a test for withdrawing money
        let mut account = BankAccount::new(500.0);
        account.withdraw(300.0);
        assert_eq!(account.balance(), 200.0);
        account.withdraw(300.0);
        assert_eq!(account.balance(), 0.0);
    }

    // Add more tests here
}
