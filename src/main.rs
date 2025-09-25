mod entries;

use entries::{fetch_entries_from_paths, fetch_entries_to_string};
use gtk4::{glib, Application, ApplicationWindow, Label};
use gtk4::{
    prelude::*, Box, Button, FlowBox, Image, ListBox, ScrolledWindow, SearchEntry, SelectionMode,
};

const APP_ID: &str = "org.gtk_rs.menu_gtk_1";

#[cfg(target_os = "linux")]
fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(ui);

    // println!("{:?}", fetch_entries_from_paths(fetch_entries_to_string()));

    app.run()
}

fn ui(app: &Application) {
    let entries = fetch_entries_from_paths(fetch_entries_to_string());
    let search = SearchEntry::new();
    let list_box = ListBox::new();

    // TODO: Extract this in a function
    list_box.set_filter_func({
        let search = search.clone();
        move |box_row| {
            if let Some(box_row) = box_row.child() {
                if let Some(flow_box) = box_row.downcast_ref::<FlowBox>() {
                    if let Some(flow_box_child) = flow_box.child_at_index(1) {
                        if let Some(uncasted_label) = flow_box_child.child() {
                            if let Some(label) = uncasted_label.downcast_ref::<Label>() {
                                let text = search.text();
                                // println!("{:?}\n", &text);
                                if label
                                    .text()
                                    .as_str()
                                    .to_lowercase()
                                    .contains(&text.as_str().to_lowercase())
                                {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
            false
        }
    });

    // TODO: Review this
    search.connect_search_changed(glib::clone!(
        #[weak]
        list_box,
        move |_| {
            list_box.invalidate_filter();
        }
    ));

    // Create every line with icon and name
    // TODO: Extract this in a function
    for entry in entries {
        let line = FlowBox::new();
        line.set_selection_mode(SelectionMode::None);
        line.set_focusable(false); // To allow to click on the children
        line.set_can_target(false); // without the click being absorbed

        // Get the icon corresponding to the icon theme
        let img = Image::from_icon_name(&entry.img_path);
        line.append(&img);

        let name = Label::new(Some(&entry.name));
        line.append(&name);

        list_box.append(&line);
    }

    // TODO: Extract this in a function
    list_box.connect_row_activated(|_list_box, row| {
        let Some(child) = row.child() else {
            println!("Row has no child");
            return;
        };

        // Try to downcast to FlowBox
        let Some(flowbox) = child.downcast_ref::<FlowBox>() else {
            println!("Row child is not a FlowBox");
            return;
        };

        // Get the second child (index 1) of the FlowBox
        let Some(flow_child) = flowbox.child_at_index(1) else {
            println!("FlowBox has no child at index 1");
            return;
        };

        // Get the widget inside that FlowBox child
        let Some(inner_child) = flow_child.child() else {
            println!("FlowBox child at index 1 has no inner child");
            return;
        };

        // Try to downcast that widget to a Label
        let Some(label) = inner_child.downcast_ref::<Label>() else {
            println!("Inner child is not a Label");
            return;
        };

        println!("Selected {}", label.text());
    });

    let scrolled_window = ScrolledWindow::builder().child(&list_box).build();
    scrolled_window.set_hexpand(true);
    scrolled_window.set_vexpand(true);

    let vbox = Box::new(gtk4::Orientation::Vertical, 5);
    vbox.append(&search);
    vbox.append(&scrolled_window);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Menu")
        .child(&vbox) // only one child now: the box
        .build();

    window.present();
}
