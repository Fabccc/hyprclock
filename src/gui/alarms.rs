use std::sync::Arc;

use gtk4::{
    prelude::{BoxExt, GridExt},
    Box, Grid, Label,
};

use crate::configuration::alarm::{AlarmConfig, AlarmsConfig};

pub fn create_alarm_list(config: &AlarmsConfig) -> Arc<Grid> {
    let row_count: i32 = 3;
    let grid = Grid::builder()
        .margin_start(5)
        .margin_end(5)
        .margin_top(5)
        .margin_bottom(5)
        .row_spacing(10)
        .column_spacing(10)
        .build();

    for (index, alarm) in config.alarms.iter().enumerate() {
        grid.attach(
            &create_alarm_item(alarm),
            (index as i32) / row_count,
            (index as i32) % row_count,
            1,
            1,
        );
    }

    Arc::new(grid)
}

fn create_alarm_item(alarm: &AlarmConfig) -> Box {
    let gtk_box = Box::builder().css_classes(vec!["alarm"]).build();

    let alarm_name = Label::builder()
        .label(alarm.get_name())
        .css_classes(vec!["alarm_name"])
        .build();
    let alarm_timer = Label::builder()
        .label("")
        .css_classes(vec!["alarm_timer"])
        .build();

    gtk_box.append(&alarm_name);
    gtk_box.append(&alarm_timer);

    gtk_box
}
