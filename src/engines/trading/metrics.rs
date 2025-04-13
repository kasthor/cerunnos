use chrono::{DateTime, Utc};

use crate::data_structures::{performance_metrics::PerformanceMetrics, trade::Trade};

pub struct Metrics {
    completed_trades: Vec<Trade>,
    equity_curve: Vec<(DateTime<Utc>, f64)>,
    drawdowns: Vec<(DateTime<Utc>, f64)>,
    max_equity: f64,

    initial_balance: f64,
    final_balance: f64,
}

impl Metrics {
    pub fn new(initial_balance: f64) -> Self {
        Self {
            initial_balance,
            final_balance: initial_balance,
            completed_trades: Vec::new(),
            equity_curve: vec![(Utc::now(), initial_balance)],
            drawdowns: vec![],
            max_equity: initial_balance,
        }
    }

    pub fn add_trade(&mut self, trade: Trade) {
        self.completed_trades.push(trade);
    }

    pub fn update_equity_curve(&mut self, time: DateTime<Utc>, balance: f64) {
        self.final_balance = balance;
        self.max_equity = f64::max(self.max_equity, balance);

        let drawdown_percent = if self.max_equity > 0.0 {
            ((self.max_equity - balance) / self.max_equity) * 100.0
        } else {
            0.0
        };

        self.equity_curve.push((time, balance));
        self.drawdowns.push((time, drawdown_percent));
    }

    pub fn sharpe_ratio(&self, risk_free_rate: f64) -> f64 {
        if self.equity_curve.len() < 2 {
            return 0.0;
        }

        let mut returns = Vec::new();

        for i in 1..self.equity_curve.len() {
            let prev_equity = self.equity_curve[i - 1].1;
            let curr_equity = self.equity_curve[i].1;
            if prev_equity > 0.0 {
                returns.push((curr_equity - prev_equity) / prev_equity);
            }
        }

        if returns.is_empty() {
            return 0.0;
        }

        let avg_return = returns.iter().sum::<f64>() / returns.len() as f64;
        let variance = returns.iter().map(|r| (r - avg_return).powi(2)).sum::<f64>() / returns.len() as f64;
        let std_dev = variance.sqrt();

        if std_dev == 0.0 {
            return 0.0;
        }

        const TRADE_PERIODS: f64 = 365.0;

        (avg_return - risk_free_rate / TRADE_PERIODS) * TRADE_PERIODS.sqrt() / (std_dev * TRADE_PERIODS.sqrt())
    }

    pub fn max_drawdown(&self) -> f64 {
        self.drawdowns.iter().map(|(_, dd)| *dd).fold(0.0, f64::max)
    }

    pub fn total_trades(&self) -> usize {
        self.completed_trades.len()
    }

    pub fn profitable_trades(&self) -> usize {
        self.completed_trades
            .iter()
            .filter(|trade| trade.profit_loss > 0.0)
            .count()
    }

    pub fn win_rate(&self) -> f64 {
        let total_trades = self.total_trades();

        if total_trades == 0 {
            0.0
        } else {
            (self.profitable_trades() as f64 / self.total_trades() as f64) * 100.0
        }
    }

    pub fn total_profit_loss(&self) -> f64 {
        self.completed_trades.iter().map(|trade| trade.profit_loss).sum::<f64>()
    }

    pub fn total_return_percent(&self) -> f64 {
        ((self.final_balance / self.initial_balance) - 1.0) * 100.0
    }

    pub fn profit_loss_averages(&self) -> (f64, f64) {
        let (profit_sum, loss_sum, profit_count, loss_count) = self.completed_trades.iter().fold(
            (0.0, 0.0, 0, 0),
            |(profit_sum, loss_sum, profit_count, loss_count), trade| {
                if trade.profit_loss > 0.0 {
                    (profit_sum + trade.profit_loss, loss_sum, profit_count + 1, loss_count)
                } else {
                    (
                        profit_sum,
                        loss_sum + trade.profit_loss.abs(),
                        profit_count,
                        loss_count + 1,
                    )
                }
            },
        );

        (
            if profit_count > 0 {
                profit_sum / profit_count as f64
            } else {
                0.0
            },
            if loss_count > 0 {
                loss_sum / loss_count as f64
            } else {
                0.0
            },
        )
    }

    pub fn risk_reward_ratio(&self) -> f64 {
        let (avg_profit, avg_loss) = self.profit_loss_averages();

        if avg_loss > 0.0 {
            avg_profit / avg_loss
        } else {
            0.0
        }
    }

    pub fn get_performance_metrics(&self) -> PerformanceMetrics {
        let (avg_profit, avg_loss) = self.profit_loss_averages();

        PerformanceMetrics {
            initial_balance: self.initial_balance,
            final_balance: self.final_balance,
            total_trades: self.total_trades(),
            profitable_trades: self.profitable_trades(),
            win_rate: self.win_rate(),
            total_profit_loss: self.total_profit_loss(),
            total_return_percent: self.total_return_percent(),
            avg_profit,
            avg_loss,
            risk_reward_ratio: self.risk_reward_ratio(),
            sharpe_ratio: self.sharpe_ratio(0.0),
            max_drawdown: self.max_drawdown(),
        }
    }
}
