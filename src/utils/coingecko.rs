use std::collections::HashMap;

use strum::IntoEnumIterator;

use crate::models::common::currency::Currency;

pub fn filter_market_data_by_currency(market_data: &HashMap<String, f64>) -> HashMap<String, f64> {
    let currency_map: HashMap<String, Currency> =
        Currency::iter().map(|i| (i.to_string(), i)).collect();

    market_data
        .iter()
        .filter_map(|(key, value)| {
            if currency_map.contains_key(&key.to_lowercase()) {
                Some((key.clone(), *value))
            } else {
                None
            }
        })
        .collect()
}
