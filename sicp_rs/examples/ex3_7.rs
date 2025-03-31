use std::{cell::RefCell, rc::Rc};

use sicp_rs::prelude::*;
fn extract_value(x: &List) -> i32 {
    x.try_as_basis_value::<i32>()
        .expect("Expected an i32 value")
        .clone()
}
fn make_account(balance: i32, passwd: String) -> impl Fn(&str, &str) -> List {
    let balance = Rc::new(RefCell::new(balance));

    let withdraw = {
        let balance = balance.clone();
        Rc::new(move |x: &List| {
            let x = extract_value(x);
            let mut b = balance.borrow_mut();
            if *b < x {
                return "Insufficient funds".to_listv();
            };
            *b -= x;
            (*b).to_listv()
        })
    };
    let deposit = {
        let balance = balance.clone();
        Rc::new(move |x: &List| {
            let x = extract_value(x);
            let mut b = balance.borrow_mut();
            *b += x;
            (*b).to_listv()
        })
    };
    let dispatch = {
        move |pass: &str, m: &str| {
            if pass != passwd.as_str() {
                return "Incorrect password".to_listv();
            }

            match m {
                "withdraw" => ClosureWrapper::new({
                    let withdraw = withdraw.clone();
                    move |x| Some(withdraw(x).to_listv())
                })
                .to_listv(),
                "deposit" => ClosureWrapper::new({
                    let deposit = deposit.clone();
                    move |x| Some(deposit(x).to_listv())
                })
                .to_listv(),
                _ => "Unknown request -- MAKE-ACCOUNT".to_listv(),
            }
        }
    };
    dispatch
}

fn make_joint(
    acc: &impl Fn(&str, &str) -> List,
    old_passwd: String,
    new_passwd: String,
) -> impl Fn(&str, &str) -> List {
    fn acc_closure(acc: &impl Fn(&str, &str) -> List, passwd: &str, cmd: &str) -> ClosureWrapper {
        acc(passwd, cmd)
            .try_as_basis_value::<ClosureWrapper>()
            .expect("Wrong linked account password")
            .clone()
    }
    let with_draw = acc_closure(acc, old_passwd.as_str(), "withdraw");
    let deposit = acc_closure(acc, old_passwd.as_str(), "deposit");

    let dispatch = {
        move |pass: &str, m: &str| {
            if pass != new_passwd.as_str() {
                return "Wrong joint account password".to_listv();
            }
            match m {
                "withdraw" => with_draw.clone().to_listv(),
                "deposit" => deposit.clone().to_listv(),
                _ => "Unknown request -- MAKE-ACCOUNT".to_listv(),
            }
        }
    };
    dispatch
}
fn handle_response(response: List, x: i32) -> List {
    response.try_as_basis_value::<ClosureWrapper>().map_or_else(
        |_| response.clone(),                           // 如果不是闭包，直接返回原值
        |closure| closure.call(&x.to_listv()).unwrap(), // 如果是闭包，调用它
    )
}
fn main() {
    let peter_acc = make_account(200, "open sesame".to_string());
    let paul_acc = make_joint(&peter_acc, "open sesame".to_string(), "rosebud".to_string());
    println!(
        "{}",
        handle_response(peter_acc("open sesame", "withdraw"), 100)
    );
    println!("{}", handle_response(paul_acc("rosebud", "withdraw"), 100));
    println!("{}", handle_response(paul_acc("rosebud", "deposit"), 10));
    println!(
        "{}",
        handle_response(paul_acc("open sesame", "withdraw"), 5)
    );
    println!("{}", handle_response(paul_acc("rosebud", "withdraw"), 20));
}
