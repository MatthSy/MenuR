use crate::activate::{activate_entry, select_item};
use crate::entries::{fetch_entries_to_string, fetch_entry_from_path, Entry};
use crate::keyboard::*;
use crate::keywords::Keywords;
use crate::list_view::IntegerObject;
use std::time::Instant;
use std::{cell::RefCell, rc::Rc};

use gtk4::{
    gio,
    glib::{self, clone},
    prelude::*,
    Application, ApplicationWindow, Box, Image, Label, ScrolledWindow, SearchEntry,
};
use gtk4::{CustomFilter, FilterListModel, ListView, SignalListItemFactory, SingleSelection};
use gtk4_layer_shell::{Layer, LayerShell};

pub(crate) fn ui(app: &Application) {
    let time = Instant::now();

    // Create the window at the beginning to gain some ms of delay
    let hbox = gtk4::Box::new(gtk4::Orientation::Vertical, 5);
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Menu")
        .default_width(400)
        .default_height(200)
        .child(&hbox)
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

    // Fetching the entries
    let entries_paths = fetch_entries_to_string();
    let entries = Rc::new(RefCell::new(vec![]));
    let keywords = Rc::new(RefCell::new(Keywords::new()));

    // Create the model to fill the listview :
    let values: Vec<IntegerObject> = vec![];
    let model = gio::ListStore::new::<IntegerObject>();
    model.extend_from_slice(&values);

    for (i, entry_path) in entries_paths.into_iter().enumerate() {
        glib::idle_add_local_full(
            glib::Priority::HIGH,
            clone!(
                #[strong]
                entry_path,
                #[strong]
                model,
                #[strong]
                i,
                #[strong]
                entries,
                #[strong]
                keywords,
                move || {
                    if let Some(entry) = fetch_entry_from_path(entry_path.clone()) {
                        keywords.borrow_mut().gen_keywords_for_entry(&entry);
                        print!("{}, ", &entry.name);
                        entries.borrow_mut().push(entry);
                        model.append(&IntegerObject::new(i as i32));
                    }

                    glib::ControlFlow::Break
                }
            ),
        );
    }

    // Create the matches
    let matches: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));

    // Sets the rows as a flowbox of label and icon
    let factory = SignalListItemFactory::new();
    factory.connect_setup(|_, item| {
        let hbox = Box::new(gtk4::Orientation::Horizontal, 30);
        hbox.set_height_request(32);

        let icon = Image::new();
        icon.set_pixel_size(28);
        let label = Label::new(None);
        hbox.append(&icon);
        hbox.append(&label);

        item.downcast_ref::<gtk4::ListItem>()
            .expect("Needs to be ListItem")
            .set_child(Some(&hbox));
    });

    factory.connect_bind(clone!(
        #[strong]
        entries,
        move |_, list_item| {
            let list_item = list_item
                .downcast_ref::<gtk4::ListItem>()
                .expect("Needs to be ListItem");

            // Get `Flowbox` from `ListItem`
            let hbox = list_item
                .child()
                .and_downcast::<Box>()
                .expect("The child has to be a `Label`.");

            // Get image inside the flowbox
            let binding0 = hbox.first_child().expect("no child where there should be");
            let icon = binding0
                .downcast_ref::<Image>()
                .expect("needs to be an image");

            // Get label inside the flowbox
            let binding1 = hbox.last_child().expect("no child where there should be");
            let label = binding1
                .downcast_ref::<Label>()
                .expect("needs to be a label");

            // Get the IntegerObject
            let binding2 = list_item.item().expect("There should be an item");
            let entry_id = binding2
                .downcast_ref::<IntegerObject>()
                .expect("Should be an IntegerObject")
                .number() as usize;
            if entry_id == 5 {
                println!("AAAAAAAAAAAAAA = {:?}", &entries);
            }
            // WARNING: JSP pq les entries disparaissent entre temps jsuis perdu frero

            // let entry = &entries.borrow()[entry_id];
            //
            // icon.set_icon_name(Some(&entry.img_path));
            // label.set_label(&entry.name);
            //
            // unsafe {
            //     hbox.set_data("name", entry.name.to_owned());
            //     hbox.set_data("num", entry_id);
            // }
        }
    ));

    let filter = CustomFilter::new(clone!(
        #[weak]
        matches,
        #[weak]
        search,
        #[strong]
        entries,
        #[upgrade_or]
        false,
        move |hbox| {
            if search.text().is_empty() {
                return true;
            }

            let entry_id = hbox
                .downcast_ref::<IntegerObject>()
                .expect("Should be an IntegerObject")
                .number() as usize;
            let entry: &Entry = &entries.borrow()[entry_id];

            matches
                .borrow()
                .iter()
                .any(|m| m == &entry.name.to_lowercase())
        }
    ));

    // Creation of the ListView from model and factory
    let filter_model = FilterListModel::new(Some(model), Some(filter.clone()));
    let selection_model = SingleSelection::new(Some(filter_model));
    selection_model.connect_selected_item_notify(|model| {
        dbg!(model.selected());
    });
    let list_view = ListView::new(Some(selection_model), Some(factory));

    list_view.connect_activate(clone!(
        #[strong]
        entries,
        #[weak]
        app,
        move |list_view, pos| {
            activate_entry(&app, &entries.borrow(), list_view, pos);
        }
    ));

    search.connect_search_changed(clone!(
        #[strong]
        filter,
        #[strong]
        matches,
        #[weak]
        list_view,
        move |search| {
            let text = search.text();
            if text.is_empty() {
                filter.changed(gtk4::FilterChange::Different);
                return;
            }
            let new_matches = keywords
                .borrow()
                .match_keywords(&text)
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            *matches.borrow_mut() = new_matches;
            filter.changed(gtk4::FilterChange::Different); // re-run the filter function

            // This focus and select the first visible item once the filter is completed
            // (Actually it runs once the main loop is idle but it should be once filter is
            // completed)
            glib::idle_add_local_once(clone!(
                #[weak]
                list_view,
                move || {
                    select_item(&list_view, false);
                }
            ));
        }
    ));

    // let scrolled_window = ScrolledWindow::builder().child(&list_box).build();
    // scrolled_window.set_vexpand(true);

    let scrolled_window = ScrolledWindow::builder()
        .child(&list_view)
        .vexpand_set(true)
        .hscrollbar_policy(gtk4::PolicyType::Never) // Disable horizontal scrolling
        .min_content_width(360)
        .build();

    scrolled_window.set_vexpand(true);

    hbox.append(&search);
    hbox.append(&scrolled_window);
    search.grab_focus();

    // This allows to press enter from the SearchEntry to start the first app in the list
    search.connect_activate(clone!(
        #[weak]
        list_view,
        #[weak]
        app,
        #[strong]
        entries,
        move |_| {
            activate_entry(&app, &entries.borrow(), &list_view, 0);
        }
    ));

    let controller_listview = make_listview_controller(app, &list_view, &search);
    let controller_search = make_search_controller(app, &list_view);
    let controller_window = make_window_controller(app, &search);
    list_view.add_controller(controller_listview);
    search.add_controller(controller_search);
    window.add_controller(controller_window);

    println!("Parsing and UI: {:?}", time.elapsed());
    // std::process::exit(0)
    std::thread::sleep(std::time::Duration::from_secs(1));
    println!(
        "\n\n\n\n\n\n\n\nBBBBBBBBBBBBBBBBBB = {}\n\n\n{:?}",
        entries.clone().borrow().len(),
        entries
    );
}
