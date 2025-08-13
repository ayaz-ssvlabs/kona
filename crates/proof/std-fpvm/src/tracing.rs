//! This module contains

use crate::io;
use alloc::{
    format,
    string::{String, ToString},
};
use tracing::{
    Event, Level, Metadata, Subscriber,
    field::{Field, Visit},
    span::{Attributes, Id, Record},
};

/// Custom [Subscriber] implementation that uses [crate::io] to write log entries to
/// [crate::FileDescriptor::StdOut].
#[derive(Debug, Clone)]
pub struct FpvmTracingSubscriber {
    min_level: Level,
}

impl FpvmTracingSubscriber {
    /// Create a new [FpvmTracingSubscriber] with the specified minimum log level.
    pub const fn new(min_level: Level) -> Self {
        Self { min_level }
    }
}

impl Subscriber for FpvmTracingSubscriber {
    fn enabled(&self, _metadata: &Metadata<'_>) -> bool {
        true
    }

    fn new_span(&self, _span: &Attributes<'_>) -> Id {
        Id::from_u64(1)
    }

    fn record(&self, _span: &Id, _values: &Record<'_>) {}

    fn record_follows_from(&self, _span: &Id, _follows: &Id) {}

    fn event(&self, event: &Event<'_>) {
        let metadata = event.metadata();
        // Comparisons for the [Level] type are inverted. See the [Level] documentation for more
        // information.
        if *metadata.level() > self.min_level {
            return;
        }

        let mut visitor = FieldVisitor::new();
        event.record(&mut visitor);
        let mut out = visitor.message;
        if !visitor.kv_pairs.is_empty() {
            out.push_str(" ");
            out.push_str(&visitor.kv_pairs.join(", "));
        }
        io::print(&format!("[{}] {}: {}", metadata.level(), metadata.target(), out));
    }

    fn enter(&self, _span: &Id) {}

    fn exit(&self, _span: &Id) {}
}

/// Custom [Visit] implementation to extract log field values.
struct FieldVisitor {
    message: String,
    kv_pairs: alloc::vec::Vec<String>,
}

impl FieldVisitor {
    const fn new() -> Self {
        Self { message: String::new(), kv_pairs: alloc::vec::Vec::new() }
    }
}

impl Visit for FieldVisitor {
    fn record_debug(&mut self, field: &Field, value: &dyn core::fmt::Debug) {
        let rendered = format!("{:?}", value);
        if field.name() == "message" {
            self.message = rendered;
        } else {
            self.kv_pairs.push(format!("{}={}", field.name(), rendered));
        }
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        if field.name() == "message" {
            self.message = value.to_string();
        } else {
            self.kv_pairs.push(format!("{}={}", field.name(), value));
        }
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        self.kv_pairs.push(format!("{}={}", field.name(), value));
    }

    fn record_i64(&mut self, field: &Field, value: i64) {
        self.kv_pairs.push(format!("{}={}", field.name(), value));
    }

    fn record_u128(&mut self, field: &Field, value: u128) {
        self.kv_pairs.push(format!("{}={}", field.name(), value));
    }

    fn record_i128(&mut self, field: &Field, value: i128) {
        self.kv_pairs.push(format!("{}={}", field.name(), value));
    }

    fn record_bool(&mut self, field: &Field, value: bool) {
        self.kv_pairs.push(format!("{}={}", field.name(), value));
    }

    // Other numeric types are coerced by tracing into i64/u64 where possible; no extra handlers needed
}
