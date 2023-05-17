use std::{f32::consts::PI, thread, time::Duration};

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

fn main() {
    let mut thermometer = Thermometer {
        base_temperature: 24.0,
        amplitude: 6.0,
        period: 120.0,
        max_measurement_error: 0.6,
        rng: rand::thread_rng(),
    };

    for i in 0.. {
        let dur = Duration::from_secs_f32(1.0);
        println!("{}", thermometer.get(dur.as_secs_f32() * (i as f32)));
        thread::sleep(dur);
    }
}
