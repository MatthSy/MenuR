mod entries;
mod keywords;

use entries::{fetch_entries_from_paths, fetch_entries_to_string};
use gtk4::{
    glib::{self, clone},
    prelude::*,
    Application, ApplicationWindow, Box, FlowBox, FlowBoxChild, Image, Label, ListBox,
    ScrolledWindow, SearchEntry,
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

    for entry in entries {
        let flow_box = FlowBox::new();
        let icon = Image::from_icon_name(&entry.img_path);
        let label = Label::new(Some(&entry.name));

        // Allows to click on the name and the icon without selecting them but only selecting the
        // line :
        let icon_row = FlowBoxChild::new();
        icon_row.set_child(Some(&icon));
        icon_row.set_can_target(false);
        let label_row = FlowBoxChild::new();
        label_row.set_child(Some(&label));
        label_row.set_can_target(false);

        flow_box.append(&icon_row);
        flow_box.append(&label_row);
        list_box.append(&flow_box);
    }

    // TODO: Change the search function to search on keywords
    let search_clone = search.clone();
    list_box.set_filter_func(move |box_row| {
        if let Some(box_row_child) = box_row.child() {
            if let Some(flow_box) = box_row_child.downcast_ref::<FlowBox>() {
                if let Some(child) = flow_box.child_at_index(1) {
                    if let Some(temp_child) = child.child() {
                        if let Some(label) = temp_child.downcast_ref::<Label>() {
                            let text = search_clone.text();
                            return label
                                .text()
                                .to_lowercase()
                                .as_str()
                                .contains(text.to_lowercase().as_str());
                        }
                    }
                }
            }
        }
        true
    });

    search.connect_search_changed(clone!(
        #[weak]
        list_box,
        move |_| {
            list_box.invalidate_filter(); // re-run the filter function
        }
    ));

    let scrolled_window = ScrolledWindow::builder().child(&list_box).build();
    scrolled_window.set_vexpand(true);

    let vbox = gtk4::Box::new(gtk4::Orientation::Vertical, 5);
    vbox.append(&search);
    vbox.append(&scrolled_window);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Menu")
        .child(&vbox)
        .build();

    window.present();
}
