use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
//use std::borrow::BorrowMut;

use crate::mod_bank::account::*;

pub struct Manager
{
    pub normal_hash : HashMap<i64, Rc<RefCell<NormalAccount>>>,
    pub high_hash : HashMap<i64, Rc<RefCell<HighCreditAccount>>>,
    pub account_list: Vec<Rc<RefCell<dyn Account>>>,
    pub list_len: i64,
}

impl Manager
{
    pub fn show_menu(&self) {
        println!(
            "--------BANK--------\n;
        1. Make Normal Account\n
        2. Make High Credit Account\n
        3. Deposit\n
        4. Withdraw\n
        5. showAccount\n
        6. Program exit\n"
        )
    }
    
    pub fn make_normal_account(&mut self)
    {
        println!("성함, 계좌번호, 초기 금액을 공백으로 구분하여 입력: ");
        let mut input : String = String::new();
        std::io::stdin().read_line(&mut input).expect("입력이 잘못됌(make normal account)");
        let tmp : Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();
        let name = tmp.get(0).unwrap();
        let acnum : i64 = tmp.get(1).unwrap().parse().expect("계좌번호 입력 실패(make normal account)");
        let balance : i64 = tmp.get(2).unwrap().parse().expect("초기 금액 입력 실패(make normal account)");
        let na = Rc::new(RefCell::new(
            NormalAccount::new(name.to_string(), acnum, balance)));
        self.account_list.push(na.clone());
        self.normal_hash.insert(acnum, na);
        self.list_len += 1;
    }
    
    pub fn make_high_credit_account(&mut self)
    {
        println!("성함, 계좌번호, 초기 금액과 Special을 공백으로 구분하여 입력: ");
        let mut input : String = String::new();
        std::io::stdin().read_line(&mut input).expect("입력이 잘못됌(make high_credit account)");
        let tmp : Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();
        let name = tmp.get(0).unwrap();
        let acnum : i64 = tmp.get(1).unwrap().parse().expect("계좌번호 입력 실패(make high_credit account)");
        let balance : f64 = tmp.get(2).unwrap().parse().expect("초기 금액 입력 실패(make high_credit account)");
        let special : f64 = tmp.get(3).unwrap().parse().expect("Special 입력 실패(make high_credit account)");
        let ha = Rc::new(RefCell::new(
            HighCreditAccount::new(name.to_string(), acnum, balance, special)));
        self.account_list.push(ha.clone());
        self.high_hash.insert(acnum, ha);
        self.list_len += 1;
        
    }
    
    pub fn deposit(&mut self)
    {
        println!("입금할 계좌번호와 입금액: ");
        let mut input : String = String::new();
        std::io::stdin().read_line(&mut input).expect("입력이 잘못됌(deposit)");
        let tmp : Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();
        let acnum : i64 = tmp.get(0).unwrap().parse().expect("deposit error");
        
        match self.normal_hash.get(&acnum)
        {
            Some(v) => {
                let dep : i64 = tmp.get(1).unwrap().parse().expect("type error");
                v.borrow_mut().deposit(dep);
                return;
            },
            None => println!(),
        };

        match self.high_hash.get(&acnum)
        {
            Some(v) => {
                let dep : f64 = tmp.get(1).unwrap().parse().expect("type error");
                v.borrow_mut().deposit(dep);
                return;
            },
            None => println!(),
        };

        println!("계좌가 존재하지 않습니다.");

    }
    
    pub fn withdraw(&mut self)
    {
        println!("출금할 계좌번호와 입금액: ");
        let mut input : String = String::new();
        std::io::stdin().read_line(&mut input).expect("입력이 잘못됌(deposit)");
        let tmp : Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();
        let acnum : i64 = tmp.get(0).unwrap().parse().expect("deposit error");
        
        match self.normal_hash.get(&acnum)
        {
            Some(v) => {
                let wd : i64 = tmp.get(1).unwrap().parse().expect("type error");
                v.borrow_mut().withdraw(wd);
                return;
            },
            None => println!(),
        };

        match self.high_hash.get(&acnum)
        {
            Some(v) => {
                let wd : f64 = tmp.get(1).unwrap().parse().expect("type error");
                v.borrow_mut().withdraw(wd);
                return;
            },
            None => println!(),
        };

        println!("계좌가 존재하지 않습니다.");
    }
    
    pub fn show_account(&self) {
        for ac in self.account_list.iter()
        {
            ac.borrow().show_info();
        }
    }
}


