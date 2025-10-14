use crate::entries::{fetch_entries_from_paths, fetch_entries_to_string, Entry};
use crate::keyboard::*;
use crate::keywords::Keywords;
use crate::list_view::IntegerObject;
use std::time::Instant;
use std::{cell::RefCell, rc::Rc};

use gtk4::{
    gio,
    glib::{self, clone},
    prelude::*,
    Application, ApplicationWindow, FlowBox, FlowBoxChild, Image, Label, ListBox, ListBoxRow,
    ScrolledWindow, SearchEntry,
};
use gtk4::{ListView, SignalListItemFactory, SingleSelection};
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
    // window.set_keyboard_mode(gtk4_layer_shell::KeyboardMode::Exclusive);
    window.set_keyboard_mode(gtk4_layer_shell::KeyboardMode::OnDemand);
    window.grab_focus();

    window.present();
    println!("Window: {:?}", time.elapsed());

    let search = SearchEntry::new();
    // Change tihs value if filtering seams slow :
    search.set_search_delay(10);

    // Fetching the entries
    let entries = fetch_entries_from_paths(fetch_entries_to_string());

    // Create the model to fill the listview :
    let values: Vec<IntegerObject> = (0..entries.len() as i32).map(IntegerObject::new).collect();
    let model = gio::ListStore::new::<IntegerObject>();
    model.extend_from_slice(&values);

    // Sets the rows as a flowbox of label and icon
    let factory = SignalListItemFactory::new();
    factory.connect_setup(|_, item| {
        let flowbox = FlowBox::new();

        let icon = Image::new();
        let label = Label::new(None);
        flowbox.append(&icon);
        flowbox.append(&label);

        item.downcast_ref::<gtk4::ListItem>()
            .expect("Needs to be ListItem")
            .set_child(Some(&flowbox));
    });

    factory.connect_bind(clone!(
        #[strong]
        entries,
        move |_, list_item| {
            let list_item = list_item
                .downcast_ref::<gtk4::ListItem>()
                .expect("Needs to be ListItem");

            // Get `Flowbox` from `ListItem`
            let flowbox = list_item
                .child()
                .and_downcast::<FlowBox>()
                .expect("The child has to be a `Label`.");

            // Get image inside the flowbox
            let binding0 = flowbox
                .child_at_index(0)
                .expect("no child where there should be");
            let binding01 = binding0.child().expect("nothing inside flowboxchild");
            let icon = binding01
                .downcast_ref::<Image>()
                .expect("needs to be an image");

            // Get label inside the flowbox
            let binding1 = flowbox
                .child_at_index(1)
                .expect("no child where there should be")
                .child()
                .expect("nothing inside flowboxchild");
            let label = binding1
                .downcast_ref::<Label>()
                .expect("needs to be a label");

            // Get the IntegerObject
            let binding2 = list_item.item().expect("There should be an item");
            let entry_id = binding2
                .downcast_ref::<IntegerObject>()
                .expect("Should be an IntegerObject");

            icon.set_icon_name(Some(&entries[entry_id.number() as usize].img_path));
            label.set_label(&entries[entry_id.number() as usize].name);
        }
    ));

    // // This iterates over entries while counting iterations in i.
    // for (i, entry) in (0_usize..).zip(entries.iter()) {
    //     let flow_box = FlowBox::new();
    //     let icon = Image::from_icon_name(&entry.img_path);
    //     let label = Label::new(Some(&entry.name));
    //
    //     // Allows to click on the name and the icon without selecting them but only selecting the
    //     // line :
    //     let icon_row = FlowBoxChild::new();
    //     icon_row.set_child(Some(&icon));
    //     icon_row.set_can_target(false);
    //     let label_row = FlowBoxChild::new();
    //     label_row.set_child(Some(&label));
    //     label_row.set_can_target(false);
    //
    //     flow_box.append(&icon_row);
    //     flow_box.append(&label_row);
    //
    //     let row = ListBoxRow::new();
    //     row.set_child(Some(&flow_box));
    //
    //     unsafe {
    //         row.set_data("name", entry.name.to_owned());
    //         row.set_data("num", i);
    //     }
    //
    //     let name = entry.name.to_owned();
    //     let path = entry.entry_path.to_owned();
    //     let app = app.clone();
    //     row.connect_activate(move |_| {
    //         println!("{:?}", name);
    //         match std::process::Command::new("dex").arg(&path).spawn() {
    //             Ok(_) => (),
    //             Err(err) => println!("An error has occured : \n{err}"),
    //         }
    //         app.quit();
    //     });

    // list_box.append(&row);
    // }

    let keywords = Keywords::from(&entries);
    let matches: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));

    // list_box.set_filter_func(clone!(
    //     #[weak]
    //     matches,
    //     #[weak]
    //     search,
    //     #[strong]
    //     entries,
    //     #[upgrade_or]
    //     false,
    //     move |row| {
    //         if search.text().is_empty() {
    //             return true;
    //         }
    //
    //         let num = unsafe { *row.data::<usize>("num").unwrap().as_ref() };
    //         let entry: &Entry = &entries[num];
    //         matches
    //             .borrow()
    //             .iter()
    //             .any(|m| m == &entry.name.to_lowercase())
    //     }
    // ));

    // search.connect_search_changed(clone!(
    //     #[weak]
    //     list_box,
    //     #[strong]
    //     matches,
    //     move |search| {
    //         let text = search.text();
    //         if text.is_empty() {
    //             list_box.invalidate_filter(); // re-run the filter function
    //             return;
    //         }
    //         let new_matches = keywords
    //             .match_keywords(&text)
    //             .iter()
    //             .map(|s| s.to_string())
    //             .collect::<Vec<String>>();
    //         *matches.borrow_mut() = new_matches;
    //         list_box.invalidate_filter(); // re-run the filter function
    //         select_visible_row_child_at_index(&list_box, 0, false, false);
    //     }
    // ));

    // let scrolled_window = ScrolledWindow::builder().child(&list_box).build();
    // scrolled_window.set_vexpand(true);

    let selection_model = SingleSelection::new(Some(model));
    let list_view = ListView::new(Some(selection_model), Some(factory));

    let scrolled_window = ScrolledWindow::builder()
        .child(&list_view)
        .vexpand_set(true)
        .hscrollbar_policy(gtk4::PolicyType::Never) // Disable horizontal scrolling
        .min_content_width(360)
        .build();

    scrolled_window.set_vexpand(true);

    vbox.append(&search);
    vbox.append(&scrolled_window);
    search.grab_focus();

    // This allows to press enter from the SearchEntry to start the first app in the list
    // search.connect_activate(clone!(
    //     #[weak]
    //     list_box,
    //     move |_| {
    //         select_visible_row_child_at_index(&list_box, 0, true, true);
    //     }
    // ));

    // let controller_window = make_window_controller(app, &list_box, &search);
    // let controller_search = make_search_controller(app, &list_box);
    // list_box.add_controller(controller_window);
    // search.add_controller(controller_search);
    // select_visible_row_child_at_index(&list_box, 0, false, false);

    println!("Parsing and UI: {:?}", time.elapsed());
    // std::thread::sleep(std::time::Duration::from_secs(2));
    // std::process::exit(0)
}
