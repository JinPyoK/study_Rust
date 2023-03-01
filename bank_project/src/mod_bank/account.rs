pub trait Account {
    fn show_info(&self);
    fn get_name(&self) -> Option<&String>;
    fn get_account_number(&self) -> Option<i64>;
}

pub struct NormalAccount {
    name: String,
    account_number: i64,
    balance: i64,
}

impl NormalAccount {
    pub fn new(name: String, account_number: i64, balance: i64) -> Self {
        NormalAccount {
            name,
            account_number,
            balance,
        }
    }

    pub fn deposit(&mut self, dep: i64)
    {
        println!(
            "현재 잔액: {}\n 입금 후 잔액: {}",
            self.balance,
            self.balance + dep
        );
        self.balance += dep;
    }

    pub fn withdraw(&mut self, wd: i64) {
        if self.balance < wd {
            println!("잔액이 부족하여 인출할 수 없습니다.");
            return;
        }
        println!(
            "현재 잔액: {}\n 인출 후 잔액: {}",
            self.balance,
            self.balance - wd
        );
        self.balance -= wd;
    }
}

impl Account for NormalAccount {
    fn show_info(&self) {
        println!(
            "Name: {}\nAccount Number: {}\n, Balance: {}\n",
            self.name, self.account_number, self.balance
        );
    }

    fn get_name(&self) -> Option<&String> {
        Some(&self.name)
    }

    fn get_account_number(&self) -> Option<i64> {
        Some(self.account_number)
    }

}

pub struct HighCreditAccount {
    name: String,
    account_number: i64,
    balance: f64,
    special: f64,
}

impl HighCreditAccount {
    pub fn new(name: String, account_number: i64, balance: f64, special: f64) -> Self {
        HighCreditAccount {
            name,
            account_number,
            balance,
            special,
        }
    }

    pub fn deposit(&mut self, dep: f64){
        println!(
            "현재 잔액: {}\n 입금 후 잔액: {}",
            self.balance,
            self.balance + dep * (1.0 + self.special)
        );
        self.balance += dep * (1.0 + self.special);
    }

    pub fn withdraw(&mut self, wd: f64) {
        if self.balance < wd {
            println!("잔액이 부족하여 인출할 수 없습니다.");
            return;
        }
        println!(
            "현재 잔액: {}\n 인출 후 잔액: {}",
            self.balance,
            self.balance - wd
        );
        self.balance -= wd;
    }
}

impl Account for HighCreditAccount {
    fn show_info(&self) {
        println!(
            "Name: {}\nAccount Number: {}\n, Balance: {}\n, Special: {}",
            self.name, self.account_number, self.balance, self.special
        );
    }

    fn get_name(&self) -> Option<&String> {
        Some(&self.name)
    }

    fn get_account_number(&self) -> Option<i64> {
        Some(self.account_number)
    }
}
