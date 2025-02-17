use std::fs;

use objective::parse_obj;

fn main() {
    let model = parse_obj(fs::read_to_string("assets/box.obj").unwrap().as_str()).unwrap();
    dbg!(model);
}
