use prometheus::{
    Encoder, Gauge, Histogram, HistogramOpts, HistogramVec, IntCounter, IntCounterVec, Opts,
    Registry, TextEncoder,
};
use std::sync::Arc;

pub struct Metrics {
    pub registry: Registry,
    pub http_requests_total: IntCounterVec,
    pub http_request_duration_seconds: HistogramVec,
    pub active_connections: Gauge,
    pub database_queries_total: IntCounter,
    pub database_query_duration_seconds: Histogram,
}

impl Metrics {
    pub fn new() -> anyhow::Result<Arc<Self>> {
        let registry = Registry::new();

        let http_requests_total = IntCounterVec::new(
            Opts::new("http_requests_total", "Total number of HTTP requests"),
            &["method", "endpoint", "status"],
        )?;
        registry.register(Box::new(http_requests_total.clone()))?;

        let http_request_duration_seconds = HistogramVec::new(
            HistogramOpts::new(
                "http_request_duration_seconds",
                "HTTP request duration in seconds",
            ),
            &["method", "endpoint"],
        )?;
        registry.register(Box::new(http_request_duration_seconds.clone()))?;

        let active_connections = Gauge::with_opts(Opts::new(
            "active_connections",
            "Number of active connections",
        ))?;
        registry.register(Box::new(active_connections.clone()))?;

        let database_queries_total = IntCounter::with_opts(Opts::new(
            "database_queries_total",
            "Total number of database queries",
        ))?;
        registry.register(Box::new(database_queries_total.clone()))?;

        let database_query_duration_seconds = Histogram::with_opts(HistogramOpts::new(
            "database_query_duration_seconds",
            "Database query duration in seconds",
        ))?;
        registry.register(Box::new(database_query_duration_seconds.clone()))?;

        Ok(Arc::new(Self {
            registry,
            http_requests_total,
            http_request_duration_seconds,
            active_connections,
            database_queries_total,
            database_query_duration_seconds,
        }))
    }

    pub fn gather(&self) -> anyhow::Result<String> {
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        let mut buffer = Vec::new();
        encoder.encode(&metric_families, &mut buffer)?;
        Ok(String::from_utf8(buffer)?)
    }
}

