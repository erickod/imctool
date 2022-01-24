use std::fmt::{Display, Result};


#[derive(Debug)]
pub enum UnitSystem {
    Imperial,
    Metric,
    Undefined 
}

impl Display for UnitSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result{
        write!(f, "{:?}", self)
    }
}