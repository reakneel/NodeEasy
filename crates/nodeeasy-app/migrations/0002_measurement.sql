CREATE TABLE score_history (
  id TEXT PRIMARY KEY NOT NULL,
  node_id TEXT NOT NULL REFERENCES nodes(id) ON DELETE CASCADE,
  total REAL NOT NULL,
  availability REAL NOT NULL,
  latency REAL NOT NULL,
  download REAL NOT NULL,
  stability REAL NOT NULL,
  freshness REAL NOT NULL,
  calculated_at TEXT NOT NULL
);
CREATE INDEX idx_score_history_node_time ON score_history(node_id, calculated_at DESC);

CREATE TABLE measurement_runs (
  id TEXT PRIMARY KEY NOT NULL,
  node_id TEXT NOT NULL REFERENCES nodes(id) ON DELETE CASCADE,
  kind TEXT NOT NULL,
  status TEXT NOT NULL,
  started_at TEXT NOT NULL,
  finished_at TEXT,
  error TEXT
);
CREATE INDEX idx_measurement_runs_node_time ON measurement_runs(node_id, started_at DESC);
