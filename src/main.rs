use log::*;
use sensors_pub::*;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::load()?;

    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", if config.debug { "debug" } else { "info" });
    }
    env_logger::builder().format_timestamp_millis().init();

    debug!("\n{:#?}", config);

    let mut sensors = config
        .sensors
        .iter()
        .map(|(k, s)| (k, <dyn Sensor>::new(s)))
        .collect::<HashMap<_, _>>();
    info!("Sensors count: {}", sensors.len());

    let publishers = config
        .publishers
        .iter()
        .map(|(k, v)| (k, <dyn Publisher>::new(&config, v).unwrap()))
        .collect::<HashMap<_, _>>();
    info!("Publishers count: {}", sensors.len());

    for (sensor_id, sensor) in &mut sensors {
        for measure_type in sensor.measure_types() {
            for publisher in publishers.values() {
                publisher.hassio_discovery(measure_type, sensor_id).await?;
            }
        }
    }

    info!("Start measure loop");
    loop {
        for (sensor_id, sensor) in &mut sensors {
            let measure = sensor.measure()?;

            for publisher in publishers.values() {
                publisher.publish(&measure, sensor_id).await?;
            }
        }
        sleep(config.interval.into()).await;
    }
}
