use std::sync::Arc;

use glib::ControlFlow::Continue;
use gtk4::{
    prelude::{GridExt, WidgetExt},
    Grid, Justification, Label,
};

use crate::configuration::config::GeneralConfig;

// Function to create the grid
pub fn create_clock_grid(clock_label: &Arc<Label>, debug_label: Option<&Arc<Label>>) -> Grid {
    let grid = Grid::builder().row_spacing(10).column_spacing(10).build();

    // Attach clock label
    grid.attach(clock_label.as_ref(), 0, 1, 2, 1);

    // Attach debug label if it exists
    if let Some(label) = debug_label {
        grid.attach(label.as_ref(), 0, 0, 2, 1);
        label.set_hexpand(true);
        label.set_vexpand(true);
    }

    clock_label.set_hexpand(true);
    clock_label.set_vexpand(true);
    clock_label.set_vexpand(true);

    grid.set_halign(gtk4::Align::Center);
    grid.set_valign(gtk4::Align::Center);

    grid
}

// Function to create the clock label
pub fn create_clock_label(config: &GeneralConfig) -> Arc<Label> {
    Arc::new(
        Label::builder()
            .label(&config.get_current_time())
            .justify(Justification::Center)
            .wrap(true)
            .wrap_mode(gtk4::pango::WrapMode::WordChar)
            .max_width_chars(-1)
            .css_classes(vec!["clock".to_string()]) // Uses CSS class for styling
            .build(),
    )
}

pub fn start_clock_update(clock_label: Arc<Label>) {
    glib::timeout_add_seconds_local(1, move || {
        if clock_label.is_visible() {
            let current_time = get_current_time();
            clock_label.set_label(&current_time);
        }
        Continue
    });
}

fn get_current_time() -> String {
    use chrono::{DateTime, Local};

    let now: DateTime<Local> = Local::now();
    now.format("%H:%M:%S").to_string()
}
