extern crate core;

use modules::scheduler;
use proc_qq::re_exports::ricq;
use proc_qq::re_exports::ricq::version::ANDROID_WATCH;
use proc_qq::Authentication::QRCode;
use proc_qq::DeviceSource::JsonFile;
use proc_qq::{run_client, ClientBuilder, ShowQR};
use std::sync::Arc;
use std::time::Duration;
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
mod modules;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing_subscriber();

    modules::init_modules().await?;
    let qsign =
        ricq::qsign::QSignClient::new("url".to_owned(), "key ".to_owned(), Duration::from_secs(60))
            .expect("qsign client build err");
    let client = ClientBuilder::new()
        .authentication(QRCode)
        .device(JsonFile("device.json".to_owned()))
        .version(&ANDROID_WATCH)
        .qsign(Some(Arc::new(qsign)))
        .schedulers(vec![scheduler::scheduler()])
        .show_rq(Some(ShowQR::OpenBySystem))
        .build()
        .await
        .unwrap();
    let client = Arc::new(client);
    let copy = client.clone();
    tokio::spawn(async move {
        println!("{}", copy.rq_client.start_time);
    });
    run_client(client).await?;
    Ok(())
}

fn init_tracing_subscriber() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(true)
                .without_time(),
        )
        .with(
            tracing_subscriber::filter::Targets::new()
                .with_target("ricq", Level::DEBUG)
                .with_target("proc_qq", Level::DEBUG)
                .with_target("proc_qq_template", Level::DEBUG),
        )
        .init();
}
