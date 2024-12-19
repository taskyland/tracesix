use napi_derive::napi;
use once_cell::sync::OnceCell;
use owo_colors::OwoColorize;
use time::macros::format_description;
use tracing::Level;
use tracing_subscriber::{
    fmt::{format::Writer, FmtContext, FormatEvent, FormatFields},
    prelude::*,
    registry::LookupSpan,
    EnvFilter,
};

static LOGGER_INIT: OnceCell<()> = OnceCell::new();

/// Configuration options for the logger
/// @param {string} [level] - Log level (trace, debug, info, warn, error). Defaults to "info"
/// @param {boolean} [json] - Whether to output logs in JSON format. Defaults to false
#[napi(object)]
pub struct LoggerOptions {
    pub level: Option<String>,
    pub json: Option<bool>,
}

#[napi]
pub struct Logger {}

struct CustomFormat {
    service_name: String,
}

impl<S, N> FormatEvent<S, N> for CustomFormat
where
    S: tracing::Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &tracing::Event<'_>,
    ) -> std::fmt::Result {
        let now = time::OffsetDateTime::now_utc();
        let date = now
            .format(format_description!(
                "[day]-[month]-[year] [hour]:[minute]:[second]"
            ))
            .unwrap();

        let level = event.metadata().level();

        write!(
            writer,
            "{}{}{}",
            "[".bright_black(),
            date.bright_blue(),
            "]".bright_black()
        )?;

        write!(
            writer,
            "{}{}{}",
            "(".bright_black(),
            self.service_name.bright_magenta(),
            ")".bright_black()
        )?;

        write!(
            writer,
            "{}{}{}",
            "[".bright_black(),
            match *level {
                Level::ERROR => "ERROR".to_string().red().to_string(),
                Level::WARN => "WARN".to_string().yellow().to_string(),
                Level::INFO => "INFO".to_string().cyan().to_string(),
                Level::DEBUG => "DEBUG".to_string().green().to_string(),
                Level::TRACE => "TRACE".to_string().purple().to_string(),
            },
            "]".bright_black()
        )?;

        writer.write_char(' ')?;
        ctx.field_format().format_fields(writer.by_ref(), event)?;
        writeln!(writer)
    }
}

#[napi]
impl Logger {
    /// Creates a new logger instance
    /// @param {string} name - The name of the service that will be displayed in logs
    /// @param {LoggerOptions} [options] - Configuration options for the logger
    /// @returns {Logger} A new Logger instance
    #[napi(constructor)]
    pub fn new(name: String, options: Option<LoggerOptions>) -> Self {
        // Initialize the global logger only once
        LOGGER_INIT.get_or_init(|| {
            let opts = options.unwrap_or(LoggerOptions {
                level: Some("info".to_string()),
                json: Some(false),
            });

            let level = opts.level.unwrap_or_else(|| "info".to_string());

            let format = CustomFormat {
                service_name: name.clone(),
            };

            let filter_layer = EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new(&level))
                .unwrap();

            let fmt_layer = tracing_subscriber::fmt::layer().event_format(format);

            let subscriber = tracing_subscriber::registry()
                .with(filter_layer)
                .with(fmt_layer);

            tracing::subscriber::set_global_default(subscriber)
                .expect("Failed to set tracing subscriber");
        });

        Self {}
    }

    /// Logs a debug message
    /// @param {string} message - The message to log at debug level
    #[napi]
    pub fn debug(&self, message: String) {
        tracing::debug!("{}", message);
    }

    /// Logs an info message
    /// @param {string} message - The message to log at info level
    #[napi]
    pub fn info(&self, message: String) {
        tracing::info!("{}", message);
    }

    /// Logs a warning message
    /// @param {string} message - The message to log at warn level
    #[napi]
    pub fn warn(&self, message: String) {
        tracing::warn!("{}", message);
    }

    /// Logs an error message
    /// @param {string} message - The message to log at error level
    #[napi]
    pub fn error(&self, message: String) {
        tracing::error!("{}", message);
    }
}
