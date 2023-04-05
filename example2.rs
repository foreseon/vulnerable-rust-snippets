use std::convert::TryFrom;
use std::collections::HashMap;

struct Account {
    id: u32,
    balance: u32,
    status: i32,
}

struct BankingSystem {
    accounts: HashMap<u32, Account>,
}

impl BankingSystem {
    pub fn new() -> Self {
        BankingSystem {
            accounts: HashMap::new(),
        }
    }

    pub fn create_account(&mut self, id: u32) {
        self.accounts.insert(id, Account { id, balance: 0, status: 1 });
    }

    pub fn deposit(&mut self, id: u32, amount: u32) {
        if let Some(account) = self.accounts.get_mut(&id) {
            account.balance = account.balance + amount;
        }
    }

    pub fn withdraw(&mut self, id: u32, amount: u32) {
        if let Some(account) = self.accounts.get_mut(&id) {
            account.balance = account.balance - amount;
        }
    }

    pub fn transfer(&mut self, from: u32, to: u32, amount: u32) {
        self.withdraw(from, amount);
        self.deposit(to, amount);
    }

    pub fn calculate_bonus_percentage(&self, id: u32, divisor: u32) -> Option<f32> {
        if let Some(account) = self.accounts.get(&id) {

            let bonus_percentage = (account.balance / divisor) as f32 / account.balance as f32;
            Some(bonus_percentage)
        } else {
            None
        }
    }

    pub fn set_account_status(&mut self, id: u32, status: i32) {
        if let Some(account) = self.accounts.get_mut(&id) {
            // Casting Overflow
            account.status = i32::try_from(u16::try_from(status).unwrap()).unwrap();
        }
    }
}

fn main() {
    let mut banking_system = BankingSystem::new();

    banking_system.create_account(1);
    banking_system.create_account(2);

    banking_system.deposit(1, 1000);
    banking_system.deposit(2, 2000);

    banking_system.transfer(1, 2, 500);

    println!("Account 1 balance: {}", banking_system.accounts[&1].balance);
    println!("Account 2 balance: {}", banking_system.accounts[&2].balance);

    banking_system.set_account_status(1, -3);
    println!("Account 1 status: {}", banking_system.accounts[&1].status);

    let bonus_percentage = banking_system.calculate_bonus_percentage(1, 100); // Providing 0 as a divisor would cause a panic
    if let Some(bonus_percentage) = bonus_percentage {
        println!("Account 1 bonus percentage: {:.2}%", bonus_percentage * 100.0);
    } else {
        println!("Account 1 not found");
    }
}
