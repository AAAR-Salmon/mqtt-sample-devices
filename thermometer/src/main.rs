use std::{f32::consts::PI, thread, time::Duration};

use clap::Parser;
use rand::{self, Rng};

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

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(help = "broker address (like `mqtt.example.com:1883`)")]
    broker: String,

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

    let mut thermometer = Thermometer {
        base_temperature: args.base_temperature,
        amplitude: args.amplitude,
        period: args.period,
        max_measurement_error: args.max_measurement_error,
        rng: rand::thread_rng(),
    };

    for tick in 0.. {
        let dur = Duration::from_secs_f32(args.send_duration);
        println!("{}", thermometer.get(args.send_duration * (tick as f32)));
        thread::sleep(dur);
    }
}
