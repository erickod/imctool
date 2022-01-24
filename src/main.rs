mod structs;
use structs::body_data::BodyData;
use structs::unit_system::UnitSystem;
use structs::utils::input_handler;

fn main() {
    let body_data = BodyData::new();
    input_handler(body_data);

}