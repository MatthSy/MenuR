use std::cell::RefCell;
use std::rc::Rc;

use gtk4::gdk::ModifierType;
use gtk4::glib::object::ObjectExt;
use gtk4::glib::{self, clone};
use gtk4::prelude::EditableExt;
use gtk4::prelude::{ApplicationExt, Cast, WidgetExt};
use gtk4::{gdk::Key, Application, EventControllerKey, ListBox};
use gtk4::{Box, ListBoxRow, SearchEntry, StateFlags};

use gtk4::{CustomFilter, FilterListModel, ListView, SignalListItemFactory, SingleSelection};

use crate::entries::Entry;
use crate::list_view::IntegerObject;

fn get_selected(lv: &ListView, entries: &Vec<Entry>) -> Option<Entry> {
    let selection = lv
        .model()
        .unwrap()
        .downcast::<gtk4::SingleSelection>()
        .unwrap();
    let selected: Option<IntegerObject> = selection
        .selected_item()
        .map(|val| val.downcast::<IntegerObject>().expect("Should be a box"));

    if let Some(id) = selected {
        Some(entries[id.number() as usize].clone())
    } else {
        None
    }
}

pub(crate) fn make_window_controller(
    app: &Application,
    lv: &ListView,
    search: &SearchEntry,
    entries: &Vec<Entry>,
) -> EventControllerKey {
    let controller = EventControllerKey::new();
    controller.connect_key_pressed(clone!(
        #[weak]
        app,
        #[weak]
        lv,
        #[weak]
        search,
        #[strong]
        entries,
        #[upgrade_or]
        glib::Propagation::Proceed,
        move |_ctrl, key, _, state| {
            let sel_entry = get_selected(&lv, &entries);
            match key {
                Key::Escape => {
                    app.quit();
                    std::process::exit(0);
                }
                Key::Up => {
                    let first_child = if let Some(tmp) = lv.first_child() {
                        if let Ok(child) = tmp.downcast::<Box>() {
                            Some(child)
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                    let first_child_id = if let Some(first_child) = first_child {
                        unsafe {
                            first_child
                                .data::<i32>("num")
                                .unwrap_or("There should be a num data")
                                .
                        }
                    } else {
                        -1
                    };
                    if sel_entry == first_child {
                        search.grab_focus();
                        return glib::Propagation::Stop;
                    }
                }
                Key::BackSpace => {
                    search.grab_focus();
                    select_visible_row_child_at_index(&lv, 0, false, false);
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
                        select_visible_row_child_at_index(&lv, 0, false, false);
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

// pub(crate) fn make_search_controller(app: &Application, lb: &ListBox) -> EventControllerKey {
//     let controller = EventControllerKey::new();
//     controller.connect_key_pressed(clone!(
//         #[weak]
//         app,
//         #[weak]
//         lb,
//         #[upgrade_or]
//         glib::Propagation::Proceed,
//         move |_ctrl, key, _, _| {
//             match key {
//                 Key::Escape => {
//                     app.quit();
//                     std::process::exit(0);
//                 }
//                 Key::Return | Key::KP_Enter | Key::ISO_Enter => {
//                     select_visible_row_child_at_index(&lb, 0, true, true);
//                 }
//                 Key::Down => {
//                     select_visible_row_child_at_index(&lb, 1, true, false);
//                 }
//                 _ => {}
//             }
//             glib::Propagation::Proceed
//         }
//     ));
//     controller
// }

pub(crate) fn select_visible_row_child_at_index(
    lv: &ListView,
    i: usize,
    focus: bool,
    activate: bool,
) -> glib::Propagation {
    if let Some(dc) = get_visible_row_at_index(lv, i) {
        if focus {
            dc.grab_focus();
        }
        dc.set_state_flags(StateFlags::SELECTED, false);
        // lv.select_row(Some(&dc));

        if activate {
            dc.activate();
        }
    }
    glib::Propagation::Stop
}

pub(crate) fn get_visible_row_at_index(
    lv: &ListView,
    index: usize,
    matches: Rc<RefCell<Vec<String>>>,
) -> Option<Box> {
    let mut i: i32 = 0;
    let mut count: usize = 0;
    // while let Some(child) = lv.row_at_index(i) {
    //     // For security
    //     if i > 1000 {
    //         break;
    //     }
    //     i += 1;
    //     let dc = child.downcast_ref::<ListBoxRow>();
    //     if dc.is_none() {
    //         eprintln!(
    //             "Could not downcast to ListBoxRow (keyboard.rs, get_visible_row_at_index function), iteration {i}"
    //         );
    //         continue;
    //     }
    //     let dc = dc.unwrap();
    //     if dc.is_visible() && dc.is_mapped() {
    //         if index == count {
    //             return Some(dc.clone());
    //         }
    //         count += 1;
    //     }
    // }
    None
}
