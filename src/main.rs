mod tests;

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
//    let cash_flows = 2000.0;
//    let months: usize = 120;
//    println!("Result: {}", future_value_stream_of_cash_flows(&vec![cash_flows; months], convert_year_to_monthly_rate(12.0)));

    println!("Result: {}", effective_annual_rate2(8.0, 2));
}

fn convert_year_to_monthly_rate(year_percentage: f64) -> f64 {
    (year_percentage / 12.0) / 100.0
}

fn interest_rate(cash_flows: f64, months: u16, desirable_amount: f64) -> f64 {
    let mut rate = 0.0;
    loop {
        rate += 0.0001;
        let result = future_value_stream_of_cash_flows(&vec![cash_flows; months as usize], rate);
        if result > desirable_amount {
            return rate * 100.0 * 12.0;
        }
    }
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

pub fn effective_annual_rate(stated_annual_rate: f64, times_compounded: u16) -> f64 {
    let monthly_interest_rate = ((stated_annual_rate / 12.0 as f64 / 100.0) * 10000.0).round() / 10000.0;
    println!("Monthly interest rate: {}", monthly_interest_rate);
    ((1.0 + monthly_interest_rate).powi(times_compounded as i32) - 1.0) * 100.0
}

fn effective_annual_rate2(stated_annual_rate: f64, times_compounded: u16) -> f64 {
    let monthly_interest_rate = ((stated_annual_rate / times_compounded as f64 / 100.0) * 10000.0).round() / 10000.0;
    println!("Monthly interest rate: {}", monthly_interest_rate);
    ((1.0 + monthly_interest_rate).powi(times_compounded as i32) - 1.0) * 100.0
}



