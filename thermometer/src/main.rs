use std::f32::consts::PI;
use std::thread;
use std::time::Duration;
use std::time::SystemTime;

use chrono::prelude::{DateTime, Utc};
use clap::Parser;
use rand::{self, Rng};
use rumqttc::Client;
use rumqttc::MqttOptions;
use rumqttc::QoS;
use serde::{Deserialize, Serialize};

struct Thermometer<T>
where
    T: Rng,
{
    base_temperature: f32,
    amplitude: f32,
    period: f32,
    max_measurement_error: f32,
    rng: T,
}

impl<T> Thermometer<T>
where
    T: Rng,
{
    fn get(&mut self, time: f32) -> f32 {
        self.base_temperature
            + self.amplitude * (2.0 * PI * time / self.period).sin()
            + self.max_measurement_error * self.rng.gen::<f32>()
    }
}

#[derive(Serialize, Deserialize)]
struct TemperatureRecord {
    timestamp: String,
    temperature: f32,
}

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(help = "broker hostname (like `mqtt.example.com`)")]
    hostname: String,

    #[arg(short, long, default_value_t = 1883)]
    port: u16,

    #[arg(short, long)]
    topic: String,

    #[arg(short, long, default_value_t = 1.0)]
    send_duration: f32,

    #[arg(long, default_value_t = 24.0)]
    base_temperature: f32,

    #[arg(long, default_value_t = 6.0)]
    amplitude: f32,

    #[arg(long, default_value_t = 120.0)]
    period: f32,

    #[arg(long, default_value_t = 0.6)]
    max_measurement_error: f32,
}

fn main() {
    let args = Args::parse();

    let mut mqttoptions = MqttOptions::new("rust-thermometer", args.hostname, args.port);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    let (mut client, mut connection) = Client::new(mqttoptions, 10);

    thread::spawn(move || {
        let mut thermometer = Thermometer {
            base_temperature: args.base_temperature,
            amplitude: args.amplitude,
            period: args.period,
            max_measurement_error: args.max_measurement_error,
            rng: rand::thread_rng(),
        };

        for tick in 0.. {
            let dur = Duration::from_secs_f32(args.send_duration);

            let temperature_record = TemperatureRecord {
                timestamp: DateTime::<Utc>::from(SystemTime::now()).to_rfc3339(),
                temperature: thermometer.get(args.send_duration * (tick as f32)),
            };
            let json = match serde_json::to_string(&temperature_record) {
                Ok(json) => json,
                Err(_) => {
                    thread::sleep(dur);
                    continue;
                },
            };

            let res_pub = client
                .publish(
                    &args.topic,
                    QoS::AtLeastOnce,
                    false,
                    json.clone(),
                );
            if let Err(err) = res_pub {
                println!("{}", err);
                thread::sleep(dur);
                continue;
            };
            println!("{}", json);

            thread::sleep(dur);
        }
    });

    for notification in connection.iter(){
        println!("Notification = {:?}", notification);
    }
}
