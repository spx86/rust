use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Timer {
    times: Vec<Duration>,
    start_time: Instant,
}

impl Timer {
    /// 创建并启动一个新的计时器
    pub fn new() -> Self {
        Self {
            times: Vec::new(),
            start_time: Instant::now(),
        }
    }

    /// 重新开始计时
    pub fn start(&mut self) {
        self.start_time = Instant::now();
    }

    /// 停止计时并记录耗时，返回本次耗时（秒）
    pub fn stop(&mut self) -> f64 {
        let elapsed = self.start_time.elapsed();
        self.times.push(elapsed);
        elapsed.as_secs_f64()
    }

    /// 返回所有计时的总和（秒）
    pub fn sum(&self) -> f64 {
        self.times.iter().map(|d| d.as_secs_f64()).sum()
    }

    /// 返回平均耗时（秒）
    pub fn avg(&self) -> f64 {
        if self.times.is_empty() {
            0.0
        } else {
            self.sum() / self.times.len() as f64
        }
    }

    /// 返回每次累计时间（cumsum）
    pub fn cumsum(&self) -> Vec<f64> {
        let mut result = Vec::with_capacity(self.times.len());
        let mut total = 0.0;
        for duration in &self.times {
            total += duration.as_secs_f64();
            result.push(total);
        }
        result
    }
}
