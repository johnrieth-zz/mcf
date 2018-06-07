pub fn debt_ratio (total_debt:f64, total_assets:f64) -> f64 {
    total_debt / total_assets
}

pub fn leverage_ratio(total_liabilities:f64,total_debts:f64,total_income:f64) -> f64 {
    (total_liabilities + total_debts) / total_income
}

pub fn rule_of_72(rate: f64) -> f64 {
    72. / (rate * 100.)
}




