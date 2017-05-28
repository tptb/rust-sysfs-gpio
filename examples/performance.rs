extern crate sysfs_gpio;

use sysfs_gpio::{Direction, Pin};
use std::time;

fn slow() -> time::Duration {
    let input = Pin::new(19);
    let mut elapsed = None;
    input.with_exported(|| {
        let now = time::Instant::now();
        input.set_direction(Direction::Out).unwrap();
        for _ in 0..1_000_000 {
            input.set_value(1).unwrap();
        }
        elapsed = Some(now.elapsed());
        Ok(())
    }).unwrap();
    elapsed.unwrap()
}

fn fast() -> time::Duration {
    let pin = Pin::new(19);
    let mut input = pin.get_fast().unwrap();
    let now = time::Instant::now();
    input.set_direction(Direction::Out).unwrap();
    for _ in 0..1_000_000 {
        input.set_value(1).unwrap();
    }
    now.elapsed()
}

fn to_millis(dur: &time::Duration) -> f64 {
    (dur.as_secs() as f64) * 1000.0 + (dur.subsec_nanos() as f64) / 1000000.0
}

fn main() {
    let slow_time = slow();
    let fast_time = fast();
    println!("slow={:?} fast={:?} fac={}", slow_time, fast_time, to_millis(&slow_time) / to_millis(&fast_time));
}
