use chrono::{DateTime, Utc};
use core::fmt;
use std::collections::{BTreeMap, HashMap};

use crate::indicators::Indicator;

use super::kline::Kline;

#[derive(Default)]
pub struct History {
    data: BTreeMap<DateTime<Utc>, Kline>,
    calculators: HashMap<String, Box<dyn Indicator>>,
    indicators: BTreeMap<DateTime<Utc>, HashMap<String, Vec<f64>>>,
}

impl fmt::Debug for History {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("History")
            .field("data", &self.data)
            .field("indicators", &self.indicators)
            .finish()
    }
}

impl History {
    pub fn new() -> Self {
        History {
            data: BTreeMap::new(),
            calculators: HashMap::new(),
            indicators: BTreeMap::new(),
        }
    }

    pub fn with_klines(klines: Vec<Kline>) -> Self {
        let data = BTreeMap::from_iter(klines.into_iter().map(|k| (k.time, k)));

        History {
            data,
            ..Default::default()
        }
    }

    pub fn insert(&mut self, kline: Kline) {
        self.data.insert(kline.time, kline.clone());

        for (name, calculator) in &self.calculators {
            let value = calculator.calculate(&self);
            self.indicators
                .entry(kline.time)
                .or_insert_with(HashMap::new)
                .insert(name.clone(), value);
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn add_calculator(&mut self, indicator: Box<dyn Indicator>) {
        self.calculators.insert(indicator.name().to_string(), indicator);
    }

    pub fn last(&self, count: usize) -> Vec<Kline> {
        let items = self.data.clone();

        items
            .into_iter()
            .rev()
            .take(count)
            .rev()
            .map(|(_, kline)| kline.clone())
            .collect()
    }

    pub fn get_indicator_values(&self, indicator: &str, count: usize) -> Vec<Vec<f64>> {
        self.indicators
            .iter()
            .rev()
            .take(count)
            .rev()
            .filter_map(|(_, indicators)| indicators.get(indicator).cloned())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::data_structures::kline::helpers::generate_klines;

    use super::*;

    #[test]
    fn test_new_history() {
        let history = History::new();
        assert_eq!(history.len(), 0);
    }

    #[test]
    fn test_insert_item() {
        let mut history = History::new();
        let klines = generate_klines(Utc::now(), 5);

        for kline in klines.clone() {
            history.insert(kline);
        }

        assert_eq!(history.len(), 5);

        for kline in klines {
            assert_eq!(history.data.get(&kline.time), Some(&kline))
        }
    }

    #[test]
    fn test_get_last_n_items() {
        let mut history = History::new();
        let klines = generate_klines(Utc::now(), 10);

        for kline in klines.clone() {
            history.insert(kline)
        }

        let last_5_klines = history.last(5);
        let expected_klines = klines[5..10].to_vec();

        assert_eq!(last_5_klines, expected_klines);
    }
}
