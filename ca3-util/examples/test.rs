use ca3_util::take_id;

fn main() {
    println!("=== take_id(100, 20) ===");
    let ids = take_id(100, 20);
    println!("{:?}", ids);
    println!("ids.len() = {}", ids.len());
}