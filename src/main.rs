fn main() {
    let cash_flows = vec![2000.0, 2000.0, 2000.0, 2000.0];
    let interest_rate = 0.06;

    println!("Result: {}", present_value_stream_of_future_even_cash_flows(1000.0, 0.04, 13));

}


fn future_value(investment: f64, interest_rate: f64, time_unit: u16) -> f64 {
    investment * (1.0 + interest_rate).powi(time_unit as i32)
}

fn present_value(future_value: f64, interest_rate: f64, time_unit: u16) -> f64 {
    future_value / (1.0 + interest_rate).powi(time_unit as i32)
}

fn future_value_stream_of_cash_flows(cash_flows: &Vec<f64>, interest_rate: f64) -> f64 {
    let mut result = 0.0;
    let mut time = cash_flows.len();

    for cash in cash_flows {
        time -= 1;
        result += cash * (1.0 + interest_rate).powi(time as i32);
    }

    result
}

fn future_value_stream_of_even_cash_flows(cash: f64, interest_rate: f64, time_unit: u16) -> f64 {
    cash * annuity_compound_factor(interest_rate, time_unit)
}

fn present_value_stream_of_even_cash_flows(final_cash: f64, interest_rate: f64, time_unit: u16) -> f64 {
    final_cash / annuity_compound_factor(interest_rate, time_unit)
}

fn annuity_compound_factor(interest_rate: f64, time_unit: u16) -> f64 {
    ((1.0 + interest_rate).powi(time_unit as i32) - 1.0) / interest_rate
}

fn annuity_discount_factor(interest_rate: f64, time_unit: u16) -> f64 {
    (1.0 - (1.0 / (1.0 + interest_rate).powi(time_unit as i32))) / interest_rate
}

fn present_value_stream_of_future_even_cash_flows(cash: f64, interest_rate: f64, time_unit: u16) -> f64 {
    cash * annuity_discount_factor(interest_rate, time_unit)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_future_value() {
        assert_eq!(46305.0, future_value(42000.0, 0.05, 2u16))
    }

    #[test]
    fn test_present_value() {
        assert_eq!(45351.47392290249, present_value(50000.0, 0.05, 2u16))
    }

    #[test]
    fn test_future_value_stream_of_cash_flows() {
        let cash_flows = vec![3000.0, 2000.0, 4000.0, 1000.0];
        assert_eq!(10877.875, future_value_stream_of_cash_flows(&cash_flows, 0.05));

        let cash_flows_2 = vec![3000.0; 4];
        assert_eq!(12930.375, future_value_stream_of_cash_flows(&cash_flows_2, 0.05));
    }

    #[test]
    fn test_future_value_stream_of_even_cash_flows() {        ;
        assert_eq!(12930.375, future_value_stream_of_even_cash_flows(3000.0, 0.05, 4));

    }

    #[test]
    fn test_present_value_stream_of_even_cash_flows() {
        assert_eq!(8973.858979663244, present_value_stream_of_even_cash_flows(1000000.0, 0.06, 35));

    }
}

