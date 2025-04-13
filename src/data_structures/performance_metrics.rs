use log::info;
use serde::Serialize;
use serde_json;

#[derive(Serialize)]
pub struct PerformanceMetrics {
    pub initial_balance: f64,
    pub final_balance: f64,
    pub total_trades: usize,
    pub profitable_trades: usize,
    pub win_rate: f64,
    pub total_profit_loss: f64,
    pub total_return_percent: f64,
    pub avg_profit: f64,
    pub avg_loss: f64,
    pub risk_reward_ratio: f64,
    pub sharpe_ratio: f64,
    pub max_drawdown: f64,
}

impl PerformanceMetrics {
    pub fn print_summary(&self) {
        info!("====== Trading Performance Summary ======");
        info!("Initial Balance: ${:.2}", self.initial_balance);
        info!("Final Balance: ${:.2}", self.final_balance);
        info!(
            "Total Return: ${:.2} ({:.2}%)",
            self.total_profit_loss, self.total_return_percent
        );
        info!("Total Trades: {}", self.total_trades);
        info!(
            "Win Rate: {:.2}% ({}/{})",
            self.win_rate, self.profitable_trades, self.total_trades
        );
        info!("Avg Profit: ${:.2}", self.avg_profit);
        info!("Avg Loss: ${:.2}", self.avg_loss);
        info!("Risk-Reward Ratio: {:.2}", self.risk_reward_ratio);
        info!("Sharpe Ratio: {:.4}", self.sharpe_ratio);
        info!("Max Drawdown: {:.2}", self.max_drawdown);
        info!("==========================================");
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }

    pub fn to_csv_row(&self) -> String {
        format!(
            "{:.2},{:.2},{},{},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},{:.4}, {:.2}",
            self.initial_balance,
            self.final_balance,
            self.total_trades,
            self.profitable_trades,
            self.win_rate,
            self.total_profit_loss,
            self.total_return_percent,
            self.avg_profit,
            self.avg_loss,
            self.risk_reward_ratio,
            self.sharpe_ratio,
            self.max_drawdown
        )
    }
}
