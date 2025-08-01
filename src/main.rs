use opentelemetry::{KeyValue, global};
use opentelemetry_otlp::{MetricExporter, Protocol, WithExportConfig};
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::metrics::{PeriodicReader, SdkMeterProvider};
use std::time::Duration;

fn init_meter_provider() -> SdkMeterProvider {
    let http_endpoint =
        std::env::var("GREPTIME_HTTP_ENDPOINT").unwrap_or_else(|_| "localhost:4000".to_string());

    let otlp_http_endpoint = format!("http://{}/v1/otlp/v1/metrics", http_endpoint);

    let exporter = MetricExporter::builder()
        .with_http()
        .with_protocol(Protocol::HttpBinary)
        .with_endpoint(otlp_http_endpoint)
        .build()
        .expect("Failed to create metric exporter");

    let reader = PeriodicReader::builder(exporter)
        .with_interval(Duration::from_secs(5))
        .build();

    let provider = SdkMeterProvider::builder()
        .with_reader(reader)
        .with_resource(
            Resource::builder()
                .with_service_name("metrics-basic-example")
                .build(),
        )
        .build();

    global::set_meter_provider(provider.clone());

    provider
}

#[tokio::main]
async fn main() {
    // Initialize the MeterProvider with the http Exporter.
    let _meter_provider = init_meter_provider();

    // Create a meter from the above MeterProvider.
    let meter = global::meter("app");

    // Create a Counter Instrument.
    let counter = meter.u64_counter("process_counter").build();

    loop {
        // Record measurements using the Counter instrument.
        counter.add(10, &[KeyValue::new("k1", "v1"), KeyValue::new("k2", "v2")]);
        tokio::time::sleep(Duration::from_millis(1000)).await;
    }
}
