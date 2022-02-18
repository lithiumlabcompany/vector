use std::time::Instant;

use super::prelude::{error_stage, error_type};
use metrics::{counter, histogram};
use mongodb::{bson, error::Error as MongoError};
use vector_core::internal_event::InternalEvent;

#[derive(Debug)]
pub struct MongoDbMetricsEventsReceived<'a> {
    pub(crate) count: usize,
    pub(crate) byte_size: usize,
    pub(crate) uri: &'a str,
}

impl<'a> InternalEvent for MongoDbMetricsEventsReceived<'a> {
    fn emit_logs(&self) {
        trace!(
            message = "Events received.",
            count = self.count,
            byte_size = self.byte_size,
            uri = self.uri,
        );
    }

    fn emit_metrics(&self) {
        counter!(
            "component_received_events_total", self.count as u64,
            "uri" => self.uri.to_owned(),
        );
        counter!(
            "component_received_event_bytes_total", self.byte_size as u64,
            "uri" => self.uri.to_owned(),
        );
        // deprecated
        counter!(
            "events_in_total", self.count as u64,
            "uri" => self.uri.to_owned(),
        );
    }
}

#[derive(Debug)]
pub(crate) struct MongoDbMetricsCollectCompleted {
    pub start: Instant,
    pub end: Instant,
}

impl InternalEvent for MongoDbMetricsCollectCompleted {
    fn emit_logs(&self) {
        debug!(message = "Collection completed.");
    }

    fn emit_metrics(&self) {
        counter!("collect_completed_total", 1);
        histogram!("collect_duration_seconds", self.end - self.start);
    }
}

pub struct MongoDbMetricsRequestError<'a> {
    pub(crate) error: MongoError,
    pub endpoint: &'a str,
}

impl<'a> InternalEvent for MongoDbMetricsRequestError<'a> {
    fn emit_logs(&self) {
        error!(
            message = "MongoDb request error.",
            endpoint = %self.endpoint,
            error = ?self.error,
            error_type = error_type::REQUEST_FAILED,
            stage = error_stage::RECEIVING,
        );
    }

    fn emit_metrics(&self) {
        counter!(
            "component_errors_total", 1,
            "error" => self.error.to_string(),
            "error_type" => error_type::REQUEST_FAILED,
            "stage" => error_stage::RECEIVING,
        );
        // deprecated
        counter!("request_errors_total", 1);
    }
}

pub struct MongoDbMetricsBsonParseError<'a> {
    pub(crate) error: bson::de::Error,
    pub(crate) endpoint: &'a str,
}

impl<'a> InternalEvent for MongoDbMetricsBsonParseError<'a> {
    fn emit_logs(&self) {
        error!(
            message = "BSON document parse error.",
            endpoint = %self.endpoint,
            error = ?self.error,
            error_type = error_type::PARSER_FAILED,
            stage = error_stage::RECEIVING,
        );
    }

    fn emit_metrics(&self) {
        counter!(
            "component_errors_total", 1,
            "error" => self.error.to_string(),
            "error_type" => error_type::PARSER_FAILED,
            "stage" => error_stage::RECEIVING,
            "endpoint" => self.endpoint.to_owned(),
        );
        // deprecated
        counter!("parse_errors_total", 1);
    }
}
