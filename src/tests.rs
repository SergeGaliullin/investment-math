#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_effective_annual_rate() {
        // 12 months (periods), compounded each month
        assert_eq!(4.032670515423953, effective_annual_rate(4.0, 12, 12.0));
        // 12 months (periods), compounded each 2 months
        assert_eq!(0.6610890000000147, effective_annual_rate(4.0, 12, 2.0));
        // 12 months (periods), compounded each 3 months
        assert_eq!(0.9932705937000241, effective_annual_rate(4.0, 12, 3.0));
        // 12 months (periods), compounded each 6 months
        assert_eq!(1.9964070521231392, effective_annual_rate(4.0, 12, 6.0));
        // 12 months (periods), compounded each 18 months
        assert_eq!(6.1095860861059, effective_annual_rate(4.0, 12, 18.0));
        // 6 months, compounded semi-annually (2 periods)
        assert_eq!(6.089999999999995, effective_annual_rate(6.0, 2, 12.0/6.0));
        // 3 months, compounded semi-annually (2 periods)
        assert_eq!(1.4889156509221957, effective_annual_rate(6.0, 2, 3.0/6.0));
        // 24 months, compounded semi-annually (2 periods)
        assert_eq!(12.550881000000015, effective_annual_rate(6.0, 2, 24.0/6.0));
        // 15 months, compounded semi-annually (2 periods)
        assert_eq!(7.66959061406336, effective_annual_rate(6.0, 2, 15.0/6.0));
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
    fn test_interest_rate()
    {
        assert_eq!(6.360000000000001, interest_rate(3000.0, 120, 500000.0));
    }
}
