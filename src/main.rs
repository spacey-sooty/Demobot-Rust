use robot_rs::{
    actuators::motors::PWMSparkMax, input::xbox::Xbox, robot_main, sensors::analog::AnalogInput,
    start::RobotResult,
};
use std::sync::{atomic::AtomicBool, Arc};

fn on_update(running: Arc<AtomicBool>) -> RobotResult {
    let mut right_motor_1 = PWMSparkMax::new(0);
    let mut right_motor_2 = PWMSparkMax::new(1);

    let mut left_motor_1 = PWMSparkMax::new(3);
    let mut left_motor_2 = PWMSparkMax::new(4);

    let driver = Xbox::new(0);

    while running.load(std::sync::atomic::Ordering::Relaxed) {
        let left_value = driver.left_y().get();
        let left_speed = if left_value.abs() > 0.05 {
            left_value
        } else {
            0.0
        };

        left_motor_1.set_speed(left_speed);
        left_motor_2.set_speed(left_speed);

        let right_value = driver.right_y().get();
        let right_speed = if right_value.abs() > 0.05 {
            right_value
        } else {
            0.0
        };

        right_motor_1.set_speed(right_speed);
        right_motor_2.set_speed(right_speed);
    }

    Ok(())
}

robot_main!(on_update);
