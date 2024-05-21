use tracing::span::{Attributes, Record};
use tracing::{Event, Id, Metadata};

// https://github.com/tokio-rs/tracing/issues/2976
// we want to set the max level hint of the subscriber
pub struct FakeSubscriber {
    pub level: tracing::level_filters::LevelFilter,
}
impl tracing::Subscriber for FakeSubscriber {
    fn max_level_hint(&self) -> Option<tracing::level_filters::LevelFilter> {
        Some(self.level)
    }
    fn enabled(&self, _metadata: &Metadata<'_>) -> bool {
        true
    }

    fn new_span(&self, _span: &Attributes<'_>) -> Id {
        unreachable!()
    }

    fn record(&self, _span: &Id, _values: &Record<'_>) {
        unreachable!()
    }

    fn record_follows_from(&self, _span: &Id, _follows: &Id) {
        unreachable!()
    }

    fn event(&self, _event: &Event<'_>) {
        unreachable!()
    }

    fn enter(&self, _span: &Id) {
        unreachable!()
    }

    fn exit(&self, _span: &Id) {
        unreachable!()
    }
}
