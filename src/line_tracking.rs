// SPDX-FileCopyrightText: 2025 Iverson Briones <ivercoder@proton.me>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use rppal::gpio::{Gpio, Level};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

pub fn line_tracking(
    left_sensor_pin: u8,
    middle_sensor_pin: u8,
    right_sensor_pin: u8,
    left_sensor_data: Arc<AtomicBool>,
    middle_sensor_data: Arc<AtomicBool>,
    right_sensor_data: Arc<AtomicBool>,
    delay: u64,
    stop: Arc<AtomicBool>,
) {
    let gpio = Gpio::new().unwrap();
    let left_sensor_input = gpio.get(left_sensor_pin).unwrap().into_input();
    let middle_sensor_input = gpio.get(middle_sensor_pin).unwrap().into_input();
    let right_sensor_input = gpio.get(right_sensor_pin).unwrap().into_input();
    while !stop.load(Ordering::Relaxed) {
        left_sensor_data.store(left_sensor_input.read() == Level::High, Ordering::Relaxed);
        middle_sensor_data.store(middle_sensor_input.read() == Level::High, Ordering::Relaxed);
        right_sensor_data.store(right_sensor_input.read() == Level::High, Ordering::Relaxed);
        thread::sleep(Duration::from_millis(delay));
    }
}
