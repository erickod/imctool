use math::round::floor;
use crate::UnitSystem;
use super::utils::round;

#[derive(Debug)]
pub struct BodyData {
    weight_in_kilograms: f64,
    height_in_centimeters: f64,
    weight_in_pounds: f64,
    height_in_feets: f64,
    height_in_inches: f64,
    calc_method: UnitSystem,
    pub bmi: f64,
}


impl BodyData {
    pub fn new() -> BodyData {
        BodyData {
            weight_in_kilograms: 0.0,
            height_in_centimeters: 0.0,
            weight_in_pounds: 0.0,
            height_in_feets:0.0,
            height_in_inches:0.0,
            calc_method:  UnitSystem::Undefined,
            bmi: 0.0,
            
        }
    }

    pub fn as_imperial(mut self, weight_in_pounds:f64, height_in_feets:f64, height_in_inches: f64) -> BodyData {
        self.calc_method = UnitSystem::Imperial;
        self.weight_in_kilograms = round(weight_in_pounds / 2.205);
        self.height_in_centimeters = floor(height_in_feets * 30.48 + height_in_inches * 2.54, 0);
        self.weight_in_pounds = weight_in_pounds;
        self.height_in_feets = height_in_feets;
        self.height_in_inches = round(height_in_inches);

        self.bmi = self.get_imc();
        self
    }

    pub fn as_metric(mut self, weight_in_kilograms:f64, height_in_centimeters: f64) -> BodyData {
        self.calc_method = UnitSystem::Metric;
        self.weight_in_kilograms = weight_in_kilograms;
        self.height_in_centimeters = height_in_centimeters;
        self.weight_in_pounds = round(weight_in_kilograms * 2.205);
        self.height_in_feets = floor(68.897637795/12.0, 0);
        self.height_in_inches = round((self.height_in_centimeters/2.54) - 12.0 * self.height_in_feets);

        self.bmi = self.get_imc();
        self
    }

    pub fn get_imc(&self) -> f64 {
        if self.bmi > 0.0 {
            return self.bmi;
        }
        (self.weight_in_kilograms / (
            self.height_in_centimeters * self.height_in_centimeters
        ) * 100.0 * 100.0 * 100.0).round() / 100.0
    }
}