use placement_center::update_config;
use protocol::protocol_fnc;

fn main() {
    println!("Get Started");
    println!("{}", protocol_fnc(100000));
    println!("{}", protocol_fnc(2));
    println!("{}", update_config());
}
