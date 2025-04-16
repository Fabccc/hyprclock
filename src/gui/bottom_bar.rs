use std::sync::{Arc, Mutex};

use gtk4::{
    prelude::{FlowBoxChildExt, WidgetExt}, Application
};

use crate::configuration::logger::AppState;

/*
*  ┌───────────────────┬──────────────────┐
*  │                   │                  │
*  │      CLOCK        │      ALARM       │
*  │                   │                  │
*  └───────────────────┴──────────────────┘
*/
pub(crate) fn create_bottom_bar(_app: &Application, state: &Arc<Mutex<AppState>>) -> gtk4::FlowBox {
    let bottom_bar = gtk4::FlowBox::new();
    bottom_bar.set_valign(gtk4::Align::Fill);
    bottom_bar.set_halign(gtk4::Align::Fill);
    bottom_bar.set_margin_top(5);
    bottom_bar.set_margin_bottom(5);
    bottom_bar.set_margin_start(5);
    bottom_bar.set_margin_end(5);
    bottom_bar.set_hexpand(true); // Make the bottom bar expand to maximum width

    bottom_bar.set_column_spacing(20);
    bottom_bar.set_homogeneous(true);

    let clock_label_fb = gtk4::FlowBoxChild::new();
    let clock_label = create_clock_label();
    clock_label.set_hexpand(true); // Make each child expand
    clock_label.set_halign(gtk4::Align::Center); // Align content to start
    clock_label_fb.set_css_classes(&["menu_clock_label_fb"]);
    clock_label_fb.set_child(Some(&clock_label));

    let alarm_label_fb = gtk4::FlowBoxChild::new();
    let alarm_label = create_alarm_label();
    alarm_label.set_hexpand(true); // Make each child expand
    alarm_label.set_halign(gtk4::Align::Center); // Align content to end
    alarm_label_fb.set_css_classes(&["menu_alarm_label_fb"]);
    alarm_label_fb.set_child(Some(&alarm_label));

    bottom_bar.append(&clock_label_fb);
    bottom_bar.append(&alarm_label_fb);
    bottom_bar.set_selection_mode(gtk4::SelectionMode::None);
    bottom_bar.set_max_children_per_line(2);
    bottom_bar.set_min_children_per_line(2);
    bottom_bar.set_css_classes(&["menu_bottom_bar"]);

    bottom_bar
}

fn create_clock_label() -> gtk4::Label {
    gtk4::Label::builder()
        .label("Clock")
        .css_classes(vec!["menu_clock_label".to_string()])
        .margin_start(10)
        .margin_end(10)
        .margin_top(5)
        .margin_bottom(5)
        .valign(gtk4::Align::Center)
        .halign(gtk4::Align::Start)
        .build()
}

fn create_alarm_label() -> gtk4::Label {
    gtk4::Label::builder()
        .label("Alarm")
        .css_classes(vec!["menu_alarm_label".to_string()])
        .margin_start(10)
        .margin_end(10)
        .margin_top(5)
        .margin_bottom(5)
        .valign(gtk4::Align::Center)
        .halign(gtk4::Align::End)
        .build()
}
