
fn main() {
    println!("Future value: {}", future_value(42000.0, 1.05, 2));
    println!("Present value: {}", present_value(50000.0, 1.05, 2));
}


fn future_value(investment: f64, interest_rate: f64, time_unit: u16) -> f64 {
    investment * interest_rate.powi(time_unit as i32)
}

fn present_value(future_value: f64, interest_rate: f64, time_unit: u16) -> f64 {
    future_value / interest_rate.powi(time_unit as i32)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_future_value() {
        assert_eq!(46305.0, future_value(42000.0, 1.05, 2u16))
    }

    #[test]
    fn test_present_value() {
        assert_eq!(45351.47392290249, present_value(50000.0, 1.05, 2u16))
    }
}

