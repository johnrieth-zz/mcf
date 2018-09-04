pub fn current_ratio (current_assets:f64, current_liabilities:f64) -> f64 {
    current_assets / current_liabilities
}

pub fn debt_ratio (total_liabilities:f64, total_assets:f64) -> f64 {
    total_liabilities / total_assets
}

pub fn per (share_price:f64, earnings_share:f64) -> f64 {
    share_price / earnings_share
}

pub fn acid_test (cash:f64, market_securities:f64, accounts_receivable:f64, current_liabilities:f64) -> f64 {
    (cash + market_securities + accounts_receivable) / current_liabilities
}

pub fn quick_ratio (liquid_assets:f64, current_liabilities:f64) -> f64 {
    liquid_assets / current_liabilities
}

pub fn profit_margin(net_income: f64, net_sales:f64) -> f64 {
    net_income / net_sales
}

pub fn market_to_book(equity_market:f64, equity_book:f64)-> f64 {
    equity_market / equity_book
}

pub fn book_to_market(equity_market:f64, equity_book:f64)-> f64 {
    equity_book / equity_market
}

pub fn roa(net_income:f64, avg_total_assets:f64) -> f64 {
    net_income / avg_total_assets
}

pub fn ebit(revenue:f64, operating_expenses:f64) -> f64 {
    revenue - operating_expenses
}

pub fn roe(net_income:f64, book_value_of_equity:f64) -> f64 {
    net_income / book_value_of_equity
}

pub fn asset_turnover(net_sales_revenue:f64, avg_total_assets:f64) -> f64 {
    net_sales_revenue / avg_total_assets
}

pub fn dol(revenue:f64, operating_expenses:f64, fixed_costs:f64) -> f64 {
    ((revenue - operating_expenses) + fixed_costs) / (revenue - operating_expenses)
}

pub fn dfn(revenue:f64, operating_expenses:f64, total_interest_expense:f64) -> f64 {
    (revenue - operating_expenses) / ((revenue - operating_expenses) - total_interest_expense)
}

pub fn shareholders_equity(total_liabilities:f64, total_assets:f64) -> f64 {
    total_liabilities - total_assets
}

pub fn r72(rate:f64) -> f64 {
    72_f64 / rate
}

/// Eckhart-McHale second-order rule
pub fn em(rate:f64) -> f64 {
    (69.3 / rate) * (200 / (200 - rate)
}


/// Third-order pade approximant
pub fn pade(rate:f64) -> f64 {
    (69.3 / rate) * ((600 + (4 * rate) / (600 + rate))
}

pub fn base_interest(rate:f64, amount:f64) -> f64 {
    amount * (1 + (rate / 100))
}
