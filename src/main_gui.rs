use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use eframe::egui;
use tokio::runtime::Runtime;

mod config;