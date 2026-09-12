use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ScoreInput {
    pub availability: f64,
    pub latency_ms: Option<f64>,
    pub download_bps: Option<f64>,
    pub stability: f64,
    pub freshness: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ScoreBreakdown {
    pub total: f64,
    pub availability: f64,
    pub latency: f64,
    pub download: f64,
    pub stability: f64,
    pub freshness: f64,
}

fn latency_component(v: Option<f64>) -> f64 {
    v.map(|ms| (1.0 - (ms / 1000.0).clamp(0.0, 1.0)).max(0.0)).unwrap_or(0.0)
}
fn download_component(v: Option<f64>) -> f64 {
    v.map(|bps| (bps / 50_000_000.0).clamp(0.0, 1.0)).unwrap_or(0.0)
}

pub fn calculate_breakdown(i: ScoreInput) -> ScoreBreakdown {
    let availability = i.availability.clamp(0.0, 1.0) * 30.0;
    let latency = latency_component(i.latency_ms) * 20.0;
    let download = download_component(i.download_bps) * 25.0;
    let stability = i.stability.clamp(0.0, 1.0) * 20.0;
    let freshness = i.freshness.clamp(0.0, 1.0) * 5.0;
    let total = (availability + latency + download + stability + freshness).clamp(0.0, 100.0);
    ScoreBreakdown { total, availability, latency, download, stability, freshness }
}

pub fn calculate(i: ScoreInput) -> f64 { calculate_breakdown(i).total }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn score_is_bounded_and_explainable() {
        let s = calculate_breakdown(ScoreInput { availability: 1.0, latency_ms: Some(20.0), download_bps: Some(50_000_000.0), stability: 1.0, freshness: 1.0 });
        assert!(s.total <= 100.0 && s.total > 90.0);
        assert!((s.total - (s.availability + s.latency + s.download + s.stability + s.freshness)).abs() < 0.001);
    }
}
