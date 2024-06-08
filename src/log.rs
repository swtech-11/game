use bevy::utils::tracing;
use bevy::{
    log::{
        tracing_subscriber::{layer::SubscriberExt, Layer},
        LogPlugin,
    },
    prelude::*,
    utils::tracing::Subscriber,
};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::sync::{Arc, Mutex};

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
        let mut file = self.file.lock().unwrap();
        let mut visitor = StringVisitor::default();
        event.record(&mut visitor);
        writeln!(file, "{}", visitor.message).expect("Failed to write to log file");
    }
}

pub struct CustomLogPlugin;

impl Plugin for CustomLogPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LogPlugin {
            update_subscriber: Some(|subscriber| {
                if let Err(err) = fs::create_dir_all("./logs") {
                    eprintln!("Failed to create logs folder: {}", err);
                }
                let file_path = format!(
                    "./logs/{}.log",
                    chrono::Utc::now().format("%Y-%m-%d_%H-%M-%S")
                );

                Box::new(subscriber.with(CustomLayer::new(file_path.as_str())))
            }),
            ..Default::default()
        });
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
