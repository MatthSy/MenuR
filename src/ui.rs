use crate::entries::{fetch_entries_from_paths, fetch_entries_to_string};
use crate::keywords::Keywords;
use std::time::Instant;
use std::{cell::RefCell, rc::Rc};

use gtk4::{
    gdk::Key,
    glib::{self, clone},
    prelude::*,
    Application, ApplicationWindow, EventControllerKey, FlowBox, FlowBoxChild, Image, Label,
    ListBox, ListBoxRow, ScrolledWindow, SearchEntry,
};
use gtk4_layer_shell::{Layer, LayerShell};

pub(crate) fn ui(app: &Application) {
    let time = Instant::now();

    // Create the window at the beginning to gain some ms of delay
    let vbox = gtk4::Box::new(gtk4::Orientation::Vertical, 5);
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Menu")
        .default_width(400)
        .default_height(200)
        .child(&vbox)
        .build();

    // Layeshell options :
    window.init_layer_shell();
    window.set_layer(Layer::Top);
    window.set_keyboard_mode(gtk4_layer_shell::KeyboardMode::Exclusive);
    window.grab_focus();

    window.present();
    println!("Window: {:?}", time.elapsed());

    let search = SearchEntry::new();
    // Change tihs value if filtering seams slow :
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

        let row = ListBoxRow::new();
        row.set_child(Some(&flow_box));

        let name = entry.name.to_owned();
        let path = entry.entry_path.to_owned();
        let app = app.clone();
        row.connect_activate(move |_| {
            println!("{:?}", name);
            match std::process::Command::new("dex").arg(&path).spawn() {
                Ok(_) => (),
                Err(err) => println!("An error has occured : \n{err}"),
            }
            app.quit();
        });

        list_box.append(&row);
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

    search.connect_search_changed(clone!(
        #[weak]
        list_box,
        #[strong]
        matches,
        move |search| {
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

    let make_controller = |app: &Application| {
        let controller = EventControllerKey::new();
        controller.connect_key_pressed(clone!(
            #[weak]
            app,
            #[upgrade_or]
            glib::Propagation::Proceed,
            move |_ctrl, key, _, _| {
                if key == Key::Escape {
                    app.quit();
                    std::process::exit(0);
                }
                glib::Propagation::Proceed
            }
        ));
        controller
    };

    let controller_window = make_controller(app);
    let controller_search = make_controller(app);

    window.add_controller(controller_window);
    search.add_controller(controller_search);

    let scrolled_window = ScrolledWindow::builder().child(&list_box).build();
    scrolled_window.set_vexpand(true);

    vbox.append(&search);
    vbox.append(&scrolled_window);
    search.grab_focus();

    println!("Parsing and UI: {:?}", time.elapsed());

    // NOTE: remove later
    // std::process::exit(1);
}
