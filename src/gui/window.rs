// src/gui/window.rs
// github.com/cvusmo/hyprclock

use crate::configuration::config::Config;
use crate::configuration::logger::*;
use crate::gui::update_window::monitor_css;
use gtk::{prelude::*, Application, ApplicationWindow, Grid, Label};
use gtk4::{self as gtk, Notebook, Widget};
use std::sync::{Arc, Mutex};

use super::{
    alarms::create_alarm_list,
    clock::{create_clock_grid, create_clock_label},
};

// Function to Build UI
pub fn build_ui(
    app: &Application,
    config: &Config, // Config is already validated and loaded (either from default or file)
    state: &Arc<Mutex<AppState>>,
    debug_mode: bool,
) -> ApplicationWindow {
    // Build UI
    log_info(state, "Building UI...");

    // Get initial width and height
    let initial_width = 400;
    let initial_height = 200;

    // Create application window
    let window = create_window(app, state, initial_width, initial_height);

    // Create clock label using validated GeneralConfig
    let clock_label = create_clock_label(&config.general);
    let alarm_list = create_alarm_list(&config.alarms);

    // Debug Mode enabled label
    let debug_label = if debug_mode {
        Some(create_debug_label())
    } else {
        None
    };

    // Create grid and set it as window child
    // let grid = create_grid(&clock_label, debug_label.as_ref());
    let notebook = create_notebook(&clock_label, debug_label.as_ref(), &alarm_list);
    window.set_child(Some(&notebook));

    // Start the timer for updating the clock label using GeneralConfig
    config
        .general
        .clone()
        .start_clock_update(clock_label.clone(), state.clone());

    // Monitor window resizing events to adjust `clock_label`
    monitor_css(&window, clock_label);

    // Window built successfully
    log_info(state, "Window built successfully.");
    window
}

// Function to create window
fn create_window(
    app: &Application,
    state: &Arc<Mutex<AppState>>,
    width: i32,
    height: i32,
) -> ApplicationWindow {
    log_info(state, "Creating application window...");
    ApplicationWindow::builder()
        .application(app)
        .title("Hyprclock")
        .resizable(true)
        .css_classes(vec!["window".to_string()]) // Uses CSS class for styling
        .default_width(width)
        .default_height(height)
        .build()
}

// Function to create the debug label
fn create_debug_label() -> Arc<Label> {
    Arc::new(
        Label::builder()
            .label("Debug")
            .css_classes(vec!["debug-label".to_string()])
            .build(),
    )
}

// Function to create a container with tabs (in gtk it's called Notebook)
fn create_notebook(
    clock_label: &Arc<Label>,
    debug_label: Option<&Arc<Label>>,
    alarm_list: &Arc<impl IsA<Widget>>,
) -> Notebook {
    let notebook = Notebook::builder()
        .hexpand(true)
        .vexpand(true)
        .halign(gtk4::Align::Center)
        .valign(gtk4::Align::Center)
        .tab_pos(gtk4::PositionType::Bottom)
        .build();

    notebook.append_page(
        &create_clock_grid(clock_label, debug_label),
        Some(&Label::builder().label("Clock").build()),
    );
    notebook.append_page(
        &**alarm_list,
        Some(&Label::builder().label("Alarm").build()),
    );

    notebook
}
