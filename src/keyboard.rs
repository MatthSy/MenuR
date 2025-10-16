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

pub(crate) fn make_listview_controller(
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
            match key {
                Key::Escape => {
                    app.quit();
                    std::process::exit(0);
                }
                Key::Up => {
                    let model = lv
                        .model()
                        .expect("There should be a model")
                        .downcast::<gtk4::SingleSelection>()
                        .expect("Could not downcast to SingleSelectin model");

                    let Some(first_in_list) = model.item(0) else {
                        return glib::Propagation::Stop;
                    };
                    let Some(selected) = model.selected_item() else {
                        return glib::Propagation::Stop;
                    };
                    if selected == first_in_list {
                        search.grab_focus();
                        return glib::Propagation::Stop;
                    }
                }
                Key::BackSpace => {
                    search.grab_focus();
                    let pos = search.text().len() as i32;
                    if state.contains(ModifierType::CONTROL_MASK) {
                        search.delete_text(0, pos);
                    } else {
                        search.delete_text(pos - 1, pos);
                    };
                    return glib::Propagation::Stop;
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

pub(crate) fn make_window_controller(
    app: &Application,
    search: &SearchEntry,
) -> EventControllerKey {
    let controller = EventControllerKey::new();
    controller.connect_key_pressed(clone!(
        #[weak]
        app,
        #[weak]
        search,
        #[upgrade_or]
        glib::Propagation::Proceed,
        move |_, key, _, state| {
            if key == Key::BackSpace {
                search.grab_focus();
                let pos = search.text().len() as i32;
                if state.contains(ModifierType::CONTROL_MASK) {
                    search.delete_text(0, pos);
                } else {
                    search.delete_text(pos - 1, pos);
                };
                return glib::Propagation::Stop;
            }
            return glib::Propagation::Proceed;
        }
    ));

    controller
}
