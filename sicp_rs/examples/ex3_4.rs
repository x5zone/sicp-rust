use std::{cell::RefCell, rc::Rc};

use sicp_rs::prelude::*;
fn extract_value(x: &List) -> i32 {
    x.try_as_basis_value::<i32>()
        .expect("Expected an i32 value")
        .clone()
}
fn make_account(balance: i32, passwd: String) -> impl FnMut(&str, &str) -> List {
    let balance = Rc::new(RefCell::new(balance));
    let mut cnt = 0;

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
    let mut handle_incorrect_password = {
        move || {
            cnt += 1;
            if cnt >= 7 {
                return "Call the cops".to_listv();
            }
            "Incorrect password".to_listv()
        }
    };
    let dispatch = {
        move |pass: &str, m: &str| {
            if pass != passwd.as_str() {
                return handle_incorrect_password();
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

fn handle_response(response: List, x: i32) -> List {
    response.try_as_basis_value::<ClosureWrapper>().map_or_else(
        |_| response.clone(),                           // 如果不是闭包，直接返回原值
        |closure| closure.call(&x.to_listv()).unwrap(), // 如果是闭包，调用它
    )
}
fn main() {
    let mut acc = make_account(100, "secret password".to_string());
    println!(
        "{}",
        handle_response(acc("secret password", "withdraw"), 40)
    );
    println!(
        "{}",
        handle_response(acc("secret password", "withdraw"), 40)
    );
    // 密码错误测试
    for _ in 0..7 {
        println!(
            "{}",
            handle_response(acc("some other password", "withdraw"), 40)
        );
    }
    println!(
        "{}",
        handle_response(acc("secret password", "withdraw"), 40)
    );
}
