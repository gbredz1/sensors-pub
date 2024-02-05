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

    for (sensor_id, sensor) in &sensors {
        for measure_type in sensor.measure_types() {
            for (publisher_id, publisher) in &publishers {
                if let Err(err) = publisher
                    .declare_sensor_measure_type(measure_type, sensor_id)
                    .await
                {
                    error!(
                        "Error add sensor ({1} -> {0}). {2}",
                        publisher_id, sensor_id, err
                    );
                }
            }
        }
    }

    info!("Start measure loop");
    loop {
        for (sensor_id, sensor) in &mut sensors {
            let measure = match sensor.measure() {
                Ok(m) => m,
                Err(err) => {
                    error!(
                        "Error reading sensor measurement ({}). {}",
                        sensor_id, err
                    );
                    continue;
                }
            };

            for (publisher_id, publisher) in &publishers {
                if let Err(err) = publisher.publish(&measure, sensor_id).await {
                    error!(
                        "Error publishing the measurement ({1} -> {0}). {2}",
                        publisher_id, sensor_id, err
                    );
                }
            }
        }
        sleep(config.interval.into()).await;
    }
}
