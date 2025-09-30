mod entries;
mod keywords;

use entries::{fetch_entries_from_paths, fetch_entries_to_string};
use keywords::Keywords;

use gtk4::{
    glib::{self, clone},
    prelude::*,
    Application, ApplicationWindow, FlowBox, FlowBoxChild, Image, Label, ListBox, ListBoxRow,
    ScrolledWindow, SearchEntry,
};

use std::time::Instant;
use std::{cell::RefCell, rc::Rc};

const APP_ID: &str = "org.gtk_rs.menu_gtk_1";

#[cfg(target_os = "linux")]
fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(ui);

    app.run()
}

fn ui(app: &Application) {
    let time = Instant::now();

    // Create the window at the beginning to gain some ms of delay
    let vbox = gtk4::Box::new(gtk4::Orientation::Vertical, 5);
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Menu")
        .child(&vbox)
        .build();

    window.present();
    println!("Window: {:?}", time.elapsed());

    let search = SearchEntry::new();
    // Modify if filtering seams slow :
    search.set_search_delay(10);

    let list_box = ListBox::new();
    list_box.set_activate_on_single_click(true);

    let entries = fetch_entries_from_paths(fetch_entries_to_string());
    for entry in &entries {
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

        // let row = ListBoxRow::new();
        // row.set_child(Some(&flow_box));
        //
        // let name = entry.name.to_owned();
        // row.connect_activate(move |_| {
        //     println!("{:?}", name);
        // });
        //
        // list_box.append(&row);

        list_box.append(&flow_box);
    }

    let keywords = Keywords::from(&entries);
    let matches: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));

    list_box.set_filter_func(clone!(
        #[weak]
        matches,
        #[weak]
        search,
        #[upgrade_or]
        false,
        move |row| {
            if search.text().is_empty() {
                return true;
            }

            let Some(box_row_child) = row.child() else {
                return true;
            };
            let Some(flow_box) = box_row_child.downcast_ref::<FlowBox>() else {
                return true;
            };
            let Some(child) = flow_box.child_at_index(1) else {
                return true;
            };
            let Some(temp_child) = child.child() else {
                return true;
            };
            let Some(label) = temp_child.downcast_ref::<Label>() else {
                return true;
            };

            let row_name = label.text().to_lowercase();
            matches.borrow().iter().any(|m| m == &row_name)
        }
    ));

    // WARNING: Doesnt activate anymore
    search.connect_search_changed(clone!(
        #[weak]
        list_box,
        #[weak]
        search,
        #[weak]
        matches,
        move |_| {
            let text = search.text();
            if text.is_empty() {
                list_box.invalidate_filter(); // re-run the filter function
                return;
            }
            let new_matches = keywords
                .match_keywords(&text)
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            *matches.borrow_mut() = new_matches;
            list_box.invalidate_filter(); // re-run the filter function
        }
    ));

    let scrolled_window = ScrolledWindow::builder().child(&list_box).build();
    scrolled_window.set_vexpand(true);

    vbox.append(&search);
    vbox.append(&scrolled_window);
    println!("Parsing and UI: {:?}", time.elapsed());
}
