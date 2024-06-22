use bevy::utils::tracing;
use bevy::{
    log::{
        tracing_subscriber::{layer::SubscriberExt, Layer},
        LogPlugin,
    },
    prelude::*,
    utils::tracing::Subscriber,
};
use state::StatePlugin;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::sync::{Arc, Mutex};

mod state;

pub struct CustomLogPlugin;

impl Plugin for CustomLogPlugin {
    fn build(&self, app: &mut App) {
        let dir = format!("./logs/{}", chrono::Utc::now().format("%Y-%m-%d_%H-%M-%S"));

        if let Err(err) = fs::create_dir_all(&dir) {
            eprintln!("Failed to create logs folder: {}", err);
            return;
        }

        let state_file_path = format!("{}/state.csv", dir);

        app.add_plugins(StatePlugin {
            file_path: state_file_path,
        });

        app.add_plugins(LogPlugin {
            level: bevy::log::Level::DEBUG,
            update_subscriber: Some(|subscriber| {
                let dir = format!("./logs/{}", chrono::Utc::now().format("%Y-%m-%d_%H-%M-%S"));
                let log_file_path = format!("{}/log.log", dir);
                Box::new(subscriber.with(CustomLayer::new(&log_file_path)))
            }),
            ..Default::default()
        });
    }
}
struct CustomLayer {
    file: Arc<Mutex<std::fs::File>>,
}

impl CustomLayer {
    fn new(file_path: &str) -> Self {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(file_path)
            .expect("Failed to open log file");

        CustomLayer {
            file: Arc::new(Mutex::new(file)),
        }
    }
}
impl<S: Subscriber> Layer<S> for CustomLayer {
    fn on_event(
        &self,
        event: &bevy::utils::tracing::Event<'_>,
        _ctx: bevy::log::tracing_subscriber::layer::Context<'_, S>,
    ) {
        let mut visitor = StringVisitor::default();
        event.record(&mut visitor);
        writeln!(self.file.lock().unwrap(), "{}", visitor.message)
            .expect("Failed to write to log file");
    }
}

#[derive(Default)]
struct StringVisitor {
    message: String,
}

impl tracing::field::Visit for StringVisitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            self.message = format!("{:?}", value);
        }
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        if field.name() == "message" {
            self.message = value.to_string();
        }
    }
}
