fn main() {
//    let cash_flows = 3000.0;
//    let months: usize = 120;
//    let mut interest_rate = 0.001;
//    let desirable_amount = 500000.0;
//
//    loop {
//        interest_rate += 0.0001;
//        let result = future_value_stream_of_cash_flows(&vec![cash_flows; months], interest_rate);
//        if result > desirable_amount {
//            println!("Monthly cash: {}", cash_flows);
//            println!("Months: {}", months);
//            println!("Interest rate: {} %", interest_rate * 100.0 * 12.0);
//            break;
//        }
//    }
    let cash_flows = 2000.0;
    let months: usize = 120;
    println!("Result: {}", future_value_stream_of_cash_flows(&vec![cash_flows; months], convert_year_to_monthly_rate(12.0)));


}

fn convert_year_to_monthly_rate(year_percentage: f64) -> f64 {
    (year_percentage / 12.0) / 100.0
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

fn present_value_stream_of_cash_flows(cash_flows: &Vec<f64>, interest_rate: f64) -> f64 {
    let mut result = 0.0;
    let mut time = cash_flows.len();
    for cash in cash_flows {
        result += cash / (1.0 + interest_rate).powi(time as i32);
        time -= 1;
    }

    result
}

fn loan_periodic_payments(loan_value: f64, interest_rate: f64, time_unit: u16) -> f64 {
    loan_value / annuity_discount_factor(interest_rate, time_unit)
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

    #[test]
    fn test_loan_periodic_payments() {
        assert_eq!(683.5034039047295, loan_periodic_payments(37150.0, 0.0033, 60));
    }

    #[test]
    fn test_convert_year_to_monthly_rate() {
        assert_eq!(0.004166666666666667, convert_year_to_monthly_rate(5.0));
    }
}

