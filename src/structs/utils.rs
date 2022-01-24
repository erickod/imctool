use std::env;
use super::body_data::BodyData;


pub fn round(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}

pub fn print_usage(){
    println!("usage:");
    println!("  - bmitool metric <weight_in_kilograms> <height_in_centimeters>");
    println!("  - bmitool imperial <weight_in_pounds> <height_in_feets> <height_in_inches>");
}

pub fn input_handler(mut body_data: BodyData) -> () {
    let args:Vec<String> = env::args().skip(1).collect();
    if args.len() <= 0 {
        print_usage();
        return ()
    }
  
    match args[0].as_str() {
        "metric" => {
            if args.len() <= 2 {
                print_usage();
                return ()
            }
            let weight_in_kilograms = &args[1].parse::<f64>();
            let height_in_centimeters = args[2].parse::<f64>();

            body_data = body_data.as_metric(
                *weight_in_kilograms.as_ref().unwrap(), 
                *height_in_centimeters.as_ref().unwrap()
            );
        },
        "imperial" => {
            if args.len() <= 3 {
                print_usage();
                return ()
            }
            let weight_in_pounds = &args[1];
            let height_in_feets = &args[2];
            let height_in_inches = &args[3];
    
            body_data = body_data.as_imperial(
                *weight_in_pounds.parse::<f64>().as_ref().unwrap(), 
                *height_in_feets.parse::<f64>().as_ref().unwrap(),
                *height_in_inches.parse::<f64>().as_ref().unwrap()
            );
        },
        _ => {
            print_usage();
        }

    };

    println!("{:?}", body_data);

    ()
}
