// SPDX-FileCopyrightText: 2025 Iverson Briones <ivercoder@proton.me>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use rppal::gpio::{Gpio, Level};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::thread;
use std::time::Duration;

pub fn servo(servo_pin: u8, period: u64, angle: Arc<AtomicU8>, stop: Arc<AtomicBool>) {
    let gpio = Gpio::new().unwrap();
    let mut servo = gpio.get(servo_pin).unwrap().into_output();
    while !stop.load(Ordering::Relaxed) {
        let pulse_width: f64 = ((angle.load(Ordering::Relaxed) as f64 * 11.0) + 500.0) / 1000000.0;
        servo
            .set_pwm(
                Duration::from_millis(period),
                Duration::from_secs_f64(pulse_width),
            )
            .unwrap();
    }
}
