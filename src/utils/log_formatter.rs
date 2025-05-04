use std::fmt;
use tracing::{Subscriber, Event};
use tracing_subscriber::fmt::{FmtContext, FormatEvent, FormatFields};
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::registry::LookupSpan;
use time::OffsetDateTime;

use crate::middleware::trace::RequestId;

pub struct CustomLogFormatter;

impl<S, N> FormatEvent<S, N> for CustomLogFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        // Timestamp and level using time-rs
        let meta = event.metadata();
        let now = OffsetDateTime::now_utc()
            .format(&time::format_description::well_known::Rfc3339)
            .unwrap_or_else(|_| "unknown_time".to_string());
        write!(writer, "{} {:<5} ", now, meta.level())?;

        // Get request_id from the current span, if present
        let request_id = ctx
            .lookup_current()
            .and_then(|span| span.extensions().get::<RequestId>().map(|rid| rid.as_str().to_owned()))
            .unwrap_or_else(|| "-".to_owned());

        // Print request_id in square brackets
        write!(writer, "[{}]: ", request_id)?;

        // Target (module path)
        write!(writer, "{}: ", meta.target())?;

        // The actual log message
        ctx.format_fields(writer.by_ref(), event)?;

        writeln!(writer)
    }
}
