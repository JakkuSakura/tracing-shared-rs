use tracing::span::{Attributes, Record};
use tracing::{Event, Id, Metadata};

pub struct SharedLogger {
    dispatch: tracing::Dispatch,
    tracing_level: tracing::level_filters::LevelFilter,
    #[cfg(feature = "log")]
    logger: &'static dyn log::Log,
    #[cfg(feature = "log")]
    level: log::LevelFilter,
}

pub fn build_shared_logger() -> SharedLogger {
    SharedLogger {
        dispatch: tracing::dispatcher::get_default(|dispatch| dispatch.clone()),
        tracing_level: tracing::level_filters::LevelFilter::current(),
        #[cfg(feature = "log")]
        logger: log::logger(),
        #[cfg(feature = "log")]
        level: log::max_level(),
    }
}
// required to set level hint
// see below
struct FakeSubscriber {
    level: tracing::level_filters::LevelFilter,
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
        todo!()
    }
}

pub fn setup_shared_logger(logger: SharedLogger) {
    // https://github.com/tokio-rs/tracing/issues/2976
    // we want to set the max level hint of the subscriber
    tracing::Dispatch::new(FakeSubscriber {
        level: logger.tracing_level,
    });
    tracing::dispatcher::set_global_default(logger.dispatch).unwrap();
    #[cfg(feature = "log")]
    log::set_logger(logger.logger).unwrap();
    #[cfg(feature = "log")]
    log::set_max_level(logger.level);
}
