const ARITHMETIC_TYPES : [&str;4]= ["integer", "rational", "javascript_number", "complex"];

fn find_index(type_tag: &str) -> i32 {
    for (i, t) in ARITHMETIC_TYPES.iter().enumerate() {
        if type_tag == *t {
            return i as i32;
        }
    }
    -1
}
fn main() {

}