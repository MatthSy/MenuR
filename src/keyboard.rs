use std::cell::RefCell;
use std::rc::Rc;

use gtk4::gdk::ModifierType;
use gtk4::gio::prelude::ListModelExt;
use gtk4::glib::object::{CastNone, ObjectExt};
use gtk4::glib::{self, clone};
use gtk4::prelude::{ApplicationExt, Cast, WidgetExt};
use gtk4::prelude::{EditableExt, SelectionModelExt};
use gtk4::{gdk::Key, Application, EventControllerKey, ListBox};
use gtk4::{Box, ListBoxRow, ScrolledWindow, SearchEntry, SelectionModel, StateFlags, Widget};

use gtk4::{CustomFilter, FilterListModel, ListView, SignalListItemFactory, SingleSelection};

use crate::activate::select_item;
use crate::entries::Entry;
use crate::list_view::IntegerObject;

fn get_selected_id(lv: &ListView) -> i32 {
    let selection = lv
        .model()
        .unwrap()
        .downcast::<gtk4::SingleSelection>()
        .unwrap();
    let selected: Option<IntegerObject> = selection
        .selected_item()
        .map(|val| val.downcast::<IntegerObject>().expect("Should be a box"));

    if let Some(id) = selected {
        id.number()
    } else {
        -2
    }
}

pub(crate) fn make_window_controller(
    app: &Application,
    lv: &ListView,
    search: &SearchEntry,
) -> EventControllerKey {
    let controller = EventControllerKey::new();
    controller.connect_key_pressed(clone!(
        #[weak]
        app,
        #[weak]
        lv,
        #[weak]
        search,
        #[upgrade_or]
        glib::Propagation::Proceed,
        move |_ctrl, key, _, state| {
            let sel_entry_id = get_selected_id(&lv);
            match key {
                Key::Escape => {
                    app.quit();
                    std::process::exit(0);
                }
                Key::Up => {
                    let first_child = if let Some(tmp) = lv.first_child() {
                        tmp.downcast::<Box>().ok()
                    } else {
                        None
                    };
                    let first_child_id = if let Some(first_child) = first_child {
                        unsafe {
                            *first_child
                                .data::<i32>("num")
                                .expect("There should be a num data")
                                .as_ref()
                        }
                    } else {
                        -1
                    };
                    if sel_entry_id == first_child_id {
                        search.grab_focus();
                        return glib::Propagation::Stop;
                    }
                }
                Key::BackSpace => {
                    search.grab_focus();
                    // select_visible_row_child_at_index(&lv, 0, false, false, matches);
                    let pos = search.text().len() as i32;
                    if state.contains(ModifierType::CONTROL_MASK) {
                        search.delete_text(0, pos);
                        return glib::Propagation::Stop;
                    } else {
                        search.delete_text(pos - 1, pos);
                        return glib::Propagation::Proceed;
                    };
                }
                _ => match key.to_unicode() {
                    // When any letter is pressed, this passes it to the SearchEntry
                    Some('A'..='Z') | Some('a'..='z') => {
                        let binding = key.name().unwrap();
                        let key_val: &str = binding.as_str();
                        search.insert_text(key_val, &mut -1);
                        search.grab_focus();
                        select_item(&lv, false);
                        let pos = search.text().len();
                        search.set_position(pos as i32);
                        return glib::Propagation::Proceed;
                    }
                    _ => {}
                },
            }
            glib::Propagation::Proceed
        }
    ));
    controller
}

pub(crate) fn make_search_controller(app: &Application, lv: &ListView) -> EventControllerKey {
    let controller = EventControllerKey::new();
    controller.connect_key_pressed(clone!(
        #[weak]
        app,
        #[weak]
        lv,
        #[upgrade_or]
        glib::Propagation::Proceed,
        move |_ctrl, key, _, _| {
            match key {
                Key::Escape => {
                    app.quit();
                    std::process::exit(0);
                }
                Key::Return | Key::KP_Enter | Key::ISO_Enter => {
                    if let Some(child) = lv.first_child() {
                        child.activate();
                    }
                }
                Key::Down => {
                    select_item(&lv, true);
                    return glib::Propagation::Stop;
                }
                _ => {}
            }
            glib::Propagation::Proceed
        }
    ));
    controller
}
