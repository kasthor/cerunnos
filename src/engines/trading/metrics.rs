use crate::data_structures::performance_metrics::PerformanceMetrics;

use super::Trading;

impl Trading {
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
        ((self.balance / self.initial_balance) - 1.0) * 100.0
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
            final_balance: self.balance,
            total_trades: self.total_trades(),
            profitable_trades: self.profitable_trades(),
            win_rate: self.win_rate(),
            total_profit_loss: self.total_profit_loss(),
            total_return_percent: self.total_return_percent(),
            avg_profit,
            avg_loss,
            risk_reward_ratio: self.risk_reward_ratio(),
        }
    }
}
