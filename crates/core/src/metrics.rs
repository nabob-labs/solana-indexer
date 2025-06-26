//! A trait for collecting and managing performance metrics within the pipeline.
//!
//! The `Metrics` trait defines a set of asynchronous methods for initializing,
//! updating, flushing, and shutting down metrics in the `solana-indexer-core`
//! framework. This trait is designed for tracking various types of metrics,
//! such as counters, gauges, and histograms, to monitor performance and
//! operational health in real time.
//!
//! ## Key Concepts
//!
//! - **Gauges**: Track the value of a particular metric at a specific point in
//!   time. Useful for monitoring values that can fluctuate, like the number of
//!   queued updates.
//! - **Counters**: Track the number of times an event has occurred, such as
//!   successful or failed update processing.
//! - **Histograms**: Measure the distribution of values, such as processing
//!   times, allowing insights into latency or response times.
//!
//! ## Implementing the Trait
//!
//! To implement `Metrics`, provide implementations for each method, typically
//! sending metrics data to a monitoring system or backend service for
//! visualization and alerting. The trait requires `async` functions, allowing
//! implementations to perform non-blocking I/O operations, such as network
//! requests or database writes.

use {crate::error::IndexerResult, async_trait::async_trait, std::sync::Arc};

#[async_trait]
pub trait Metrics: Send + Sync {
    /// Initializes the metrics system, preparing it for data collection.
    async fn initialize(&self) -> IndexerResult<()>;
    /// Flushes any buffered metrics data to ensure all metrics are reported.
    async fn flush(&self) -> IndexerResult<()>;
    /// Shuts down the metrics system, performing cleanup and ensuring all data
    /// is flushed.
    async fn shutdown(&self) -> IndexerResult<()>;

    /// Updates a gauge metric, setting its value to represent the current
    /// state.
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the gauge metric to update.
    /// - `value`: The current value of the gauge metric.
    async fn update_gauge(&self, name: &str, value: f64) -> IndexerResult<()>;

    /// Increments a counter metric by a specified value.
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the counter metric to increment.
    /// - `value`: The amount by which to increment the counter.
    async fn increment_counter(&self, name: &str, value: u64) -> IndexerResult<()>;

    /// Records a value in a histogram metric, representing distribution data.
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the histogram metric to record.
    /// - `value`: The value to add to the histogram, typically representing
    ///   time or size.
    async fn record_histogram(&self, name: &str, value: f64) -> IndexerResult<()>;
}

#[derive(Default)]
pub struct MetricsCollection {
    pub metrics: Vec<Arc<dyn Metrics>>,
}

impl MetricsCollection {
    pub fn new(metrics: Vec<Arc<dyn Metrics>>) -> Self {
        Self { metrics }
    }

    pub async fn initialize_metrics(&self) -> IndexerResult<()> {
        for metric in &self.metrics {
            metric.initialize().await?;
        }
        Ok(())
    }

    pub async fn shutdown_metrics(&self) -> IndexerResult<()> {
        for metric in &self.metrics {
            metric.shutdown().await?;
        }
        Ok(())
    }

    pub async fn flush_metrics(&self) -> IndexerResult<()> {
        for metric in &self.metrics {
            metric.flush().await?;
        }
        Ok(())
    }

    pub async fn update_gauge(&self, name: &str, value: f64) -> IndexerResult<()> {
        for metric in &self.metrics {
            metric.update_gauge(name, value).await?;
        }
        Ok(())
    }

    pub async fn increment_counter(&self, name: &str, value: u64) -> IndexerResult<()> {
        for metric in &self.metrics {
            metric.increment_counter(name, value).await?;
        }
        Ok(())
    }

    pub async fn record_histogram(&self, name: &str, value: f64) -> IndexerResult<()> {
        for metric in &self.metrics {
            metric.record_histogram(name, value).await?;
        }
        Ok(())
    }
}
