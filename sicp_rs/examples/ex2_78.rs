use sicp_rs::ch2::ch2_5::*;
use sicp_rs::ch3::ch3_3::make_table_2d;
use sicp_rs::prelude::*;
fn main() {
    // 创建操作符表
    let optable = make_table_2d();
    let op_cloned = optable.clone();
    let get = move |args: List| optable("lookup").call(&args);
    let put = move |args: List| op_cloned("insert").call(&args);

    install_javascript_number_package(put.clone());
    println!("{}", add(&1.0.to_listv(), &2.0.to_listv(), get, &List::Nil));
}
