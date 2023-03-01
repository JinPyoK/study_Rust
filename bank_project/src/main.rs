mod mod_bank;

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use mod_bank::account::*;
use mod_bank::bank_manager::*;

fn main() {
    let entire_list : Vec<Rc<RefCell<dyn Account>>> = Vec::new();
    let norm_hash : HashMap<i64, Rc<RefCell<NormalAccount>>> = HashMap::new();
    let hi_hash : HashMap<i64, Rc<RefCell<HighCreditAccount>>> = HashMap::new();
    let manager = &mut Manager {
        account_list : entire_list,
        normal_hash : norm_hash,
        high_hash : hi_hash,
        list_len : 0,
    };

    manager.show_menu();
    loop {
        println!("선택: ");
        let mut input : String = String::new();
        std::io::stdin().read_line(&mut input).expect("입력이 잘못됌(make normal account)");
        let num : i64;
        match input.trim().parse()
        {
            Ok(v) => num = v,
            Err(e) => {println!("잘못된 숫자입니다. [{}]", e);continue},
        }
        //let num : i64 = input.trim().parse().expect("sadf");

        match num{
            1 => manager.make_normal_account(),
            2 => manager.make_high_credit_account(),
            3 => manager.deposit(),
            4 => manager.withdraw(),
            5 => manager.show_account(),
            6 => {println!("프로그램을 종료합니다."); return;}
            _ => println!("잘못된 선택입니다."),
        }
    }
}

