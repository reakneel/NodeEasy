use serde::Serialize;
use tokio::sync::broadcast;

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum AppEvent {
    JobStarted { job_id: String, kind: String },
    JobProgress { job_id: String, completed: u64, total: u64 },
    JobFinished { job_id: String, success: bool },
    SourceSynced { source_id: String, parsed: usize },
    NodeTested { node_id: String, success: bool, latency_ms: Option<f64> },
}

#[derive(Clone)]
pub struct EventBus { tx: broadcast::Sender<AppEvent> }
impl EventBus {
    pub fn new(capacity: usize) -> Self { let (tx, _) = broadcast::channel(capacity); Self { tx } }
    pub fn publish(&self, event: AppEvent) { let _ = self.tx.send(event); }
    pub fn subscribe(&self) -> broadcast::Receiver<AppEvent> { self.tx.subscribe() }
}
