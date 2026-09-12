use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ScoreInput {
    pub availability: f64,
    pub latency_ms: Option<f64>,
    pub download_bps: Option<f64>,
    pub stability: f64,
    pub freshness: f64,
}

pub fn calculate(i: ScoreInput) -> f64 {
    let latency = i
        .latency_ms
        .map(|v| (1.0 - (v / 1000.0).min(1.0)).max(0.0))
        .unwrap_or(0.0);
    let speed = i
        .download_bps
        .map(|v| (v / 50_000_000.0).min(1.0))
        .unwrap_or(0.0);
    (i.availability.clamp(0.0, 1.0) * 30.0
        + latency * 20.0
        + speed * 25.0
        + i.stability.clamp(0.0, 1.0) * 20.0
        + i.freshness.clamp(0.0, 1.0) * 5.0)
        .clamp(0.0, 100.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_is_bounded() {
        let s = calculate(ScoreInput {
            availability: 1.0,
            latency_ms: Some(20.0),
            download_bps: Some(50_000_000.0),
            stability: 1.0,
            freshness: 1.0,
        });
        assert!(s <= 100.0 && s > 90.0);
    }
}
