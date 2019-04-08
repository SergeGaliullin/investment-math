#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_effective_annual_rate() {
        assert_eq!(4.032670515423953, effective_annual_rate(4.0, 12))
    }

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
    fn test_future_value_stream_of_even_cash_flows() {
        ;
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

    #[test]
    fn test_interest_rate() {
        assert_eq!(6.360000000000001, interest_rate(3000.0, 120, 500000.0));
    }
}
