mod entries;
mod keywords;

use entries::{fetch_entries_from_paths, fetch_entries_to_string};
use gtk4::{
    glib::{self, clone},
    prelude::*,
    Application, ApplicationWindow, FlowBox, FlowBoxChild, Image, Label, ListBox, ScrolledWindow,
    SearchEntry,
};
use keywords::Keywords;
use std::{
    ops::Add,
    time::{Duration, Instant},
};

use std::{cell::RefCell, rc::Rc};

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
    // Modify if filtering seams slow :
    search.set_search_delay(10);
    let list_box = ListBox::new();

    let time = Instant::now();
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
        list_box.append(&flow_box);
    }

    let keywords = Keywords::from(&entries);
    let matches: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));

    // TODO: Clean this shit up
    let matches_clone = matches.clone();
    let search_clone = search.clone();

    let total_time: Rc<RefCell<Duration>> = Rc::new(RefCell::new(Duration::default()));
    let total_time1 = total_time.clone();

    list_box.set_filter_func(move |row| {
        if search_clone.text().is_empty() {
            return true;
        }
        if let Some(box_row_child) = row.child() {
            if let Some(flow_box) = box_row_child.downcast_ref::<FlowBox>() {
                if let Some(child) = flow_box.child_at_index(1) {
                    if let Some(temp_child) = child.child() {
                        if let Some(label) = temp_child.downcast_ref::<Label>() {
                            let row_name = label.text().to_lowercase();
                            let mut temp = total_time1.borrow_mut();
                            *temp = temp.add(time.elapsed());
                            return matches_clone.borrow().iter().any(|m| m == &row_name);
                        }
                    }
                }
            }
        }
        let mut temp = total_time1.borrow_mut();
        *temp = temp.add(time.elapsed());
        false
    });

    let matches_clone = matches.clone();
    search.connect_search_changed(clone!(
        #[weak]
        list_box,
        #[weak]
        search,
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
            *matches_clone.borrow_mut() = new_matches;
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
