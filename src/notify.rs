use crate::cmd::AppArgs;
use notify_rust::{Notification, NotificationHandle};
#[cfg(all(unix, not(target_os = "macos")))]
use std::time::Duration;

fn next_pomodoro(pomodoro_duration: u32, break_duration: u32) {
    let countdown_duration: u32 = 3;
    let mut notification = Notification::new()
        .summary("Work!")
        .icon("clock")
        .timeout(8_000)
        .show()
        .unwrap();

    countdown(&mut notification, "get ready: ", countdown_duration);
    notification
        .body(&format!("remaining time: {} minutes", pomodoro_duration))
        .appname("Pomodoro");
    notification.update();

    std::thread::sleep(Duration::from_secs(u64::from(
        pomodoro_duration * 60 - countdown_duration,
    )));

    let mut notification = Notification::new()
        .summary("Break!")
        .icon("clock")
        .timeout(8_000)
        .show()
        .unwrap();

    countdown(&mut notification, "get ready: ", countdown_duration);
    notification
        .body(&format!("remaining time: {} minutes", pomodoro_duration))
        .appname("Break");
    notification.update();

    std::thread::sleep(Duration::from_secs(u64::from(
        break_duration * 60 - countdown_duration,
    )));
}

// this is a xdg only feature
#[cfg(any(target_os = "windows", target_os = "macos"))]
fn countdown(_notification: Notification, _message: &str, countdown_duration: u32) {
    std::thread::sleep(Duration::from_secs(countdown_duration));
}

#[cfg(all(unix, not(target_os = "macos")))]
fn countdown(notification: &mut NotificationHandle, message: &str, countdown_duration: u32) {
    for i in 0..countdown_duration {
        notification
            .body(&format!("{}{}", message, countdown_duration - i))
            .appname(&format!("countdown_{}", countdown_duration - i));
        notification.update();
        std::thread::sleep(Duration::from_millis(1_000));
    }
}

pub fn pomodoros_launch(args: AppArgs) {
    let AppArgs {
        pomodoro_duration,
        break_duration,
        number_of_pomodoros,
    } = args;

    for _ in 0..number_of_pomodoros {
        next_pomodoro(pomodoro_duration, break_duration);
    }
}
