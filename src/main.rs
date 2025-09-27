mod entries;

use entries::{fetch_entries_from_paths, fetch_entries_to_string};
use gtk4::{glib, Application, ApplicationWindow};
use gtk4::{prelude::*, Button, ListBox, ScrolledWindow};

const APP_ID: &str = "org.gtk_rs.menu_gtk_1";

#[cfg(target_os = "linux")]
fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(ui);

    // println!("{:?}", fetch_entries_from_paths(fetch_entries_to_string()));

    app.run()
}

fn ui(app: &Application) {
    // let fetch_entry_time = std::time::SystemTime::now();
    let entries = fetch_entries_from_paths(fetch_entries_to_string());
    // for entry in entries.clone() {
    //     println!("{:?}", entry);
    // }

    let list_box = ListBox::new().set_filter_func(|box_row| {
        let entry = box_row.child();
        if entry.is_some() && entry.unwrap() {}
    });

    // Entries are consumed in this loop
    for entry in entries {
        let button = Button::from_icon_name(&entry.img_path);
        // button.set_label(&entry.name);
        // Gives ownership of the entry's data to its button
        button.connect_clicked(move |content| {
            println!(
                "Button clicked : {:?} -> {:?}",
                content.label(),
                entry.entry_path
            );
        });
        list_box.append(&button);
    }
    let scrolled_window = ScrolledWindow::builder().child(&list_box).build();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Menu")
        .child(&scrolled_window)
        .build();

    window.present();
}
