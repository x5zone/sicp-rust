use sicp_rs::prelude::*;

fn make_from_mag_ang(mag: List, ang: List) -> ClosureWrapper {
    let dispatch = move |op: &List| {
        let op = op.clone();
        if op == "real-part".to_listv() {
            println!("real-part");
            //mag * Math.cos(ang)
            let mag = mag.try_as_basis_value::<f64>().unwrap();
            let ang = ang.try_as_basis_value::<f64>().unwrap();
            Some((mag * ang.cos()).to_listv())
        } else if op == "imag-part".to_listv() {
            println!("imag-part");
            //mag * Math.sin(ang)
            let mag = mag.try_as_basis_value::<f64>().unwrap();
            let ang = ang.try_as_basis_value::<f64>().unwrap();
            Some((mag * ang.cos()).to_listv())
        } else if op == "magnitude".to_listv() {
            println!("magnitude");
            Some(mag.clone()) // 这里clone是为了避免所有权问题，若移动所有权，则closure会变为FnOnce类型
        } else if op == "angle".to_listv() {
            println!("angle");
            Some(ang.clone())
        } else {
            panic!("Unknown op -- MAKE_FROM_mag_ang {}", op)
        }
    };
    ClosureWrapper::new(dispatch)
}

fn main() {
    let x = make_from_mag_ang(1.0.to_listv(), 2.0.to_listv());
    x.call(&"real-part".to_listv());
    x.call(&"imag-part".to_listv());
    x.call(&"magnitude".to_listv());
    x.call(&"angle".to_listv());
}
