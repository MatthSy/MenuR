use gtk4::prelude::Cast;
use gtk4::prelude::SelectionModelExt;
use gtk4::prelude::WidgetExt;
use gtk4::{
    Application, ListView, SingleSelection,
    gio::prelude::{ApplicationExt, ListModelExt},
    glib::object::CastNone,
};

use crate::{entries::Entry, list_view::IntegerObject};

pub(crate) fn activate_entry(app: &Application, entries: &[Entry], list_view: &ListView, pos: u32) {
    let id = list_view
        .model()
        .expect("There should be a model")
        .item(pos)
        .and_downcast::<IntegerObject>()
        .expect("There should be an IntegerObject here")
        .number();
    let entry = &entries[id as usize];
    match std::process::Command::new("dex")
        .arg(&entry.entry_path)
        .spawn()
    {
        Ok(_) => (),
        Err(err) => {
            println!(
                "An error has occured while starting {}. Err : \n{err}",
                entry.name
            )
        }
    }
    app.quit();
    std::process::exit(0);
}

pub(crate) fn select_item(lv: &ListView, second: bool) {
    let model = lv
        .model()
        .expect("There should be a model here")
        .downcast::<SingleSelection>()
        .expect("The model should be castable to SelectionModel");
    // This gets the second child :
    if let Some(first) = lv.first_child() {
        if second && let Some(second) = first.next_sibling() {
            model.select_item(1, true);
            second.grab_focus();
        } else {
            model.select_item(0, true);
            first.grab_focus();
        }
    }
}
