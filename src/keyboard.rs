use gtk4::glib::{self, clone};
use gtk4::prelude::{ApplicationExt, Cast, WidgetExt};
use gtk4::{gdk::Key, Application, EventControllerKey, ListBox};
use gtk4::{ListBoxRow, SearchEntry, StateFlags};

pub(crate) fn make_window_controller(
    app: &Application,
    lb: &ListBox,
    search: &SearchEntry,
) -> EventControllerKey {
    let controller = EventControllerKey::new();
    controller.connect_key_pressed(clone!(
        #[weak]
        app,
        #[weak]
        lb,
        #[weak]
        search,
        #[upgrade_or]
        glib::Propagation::Proceed,
        move |_ctrl, key, _, _| {
            match key {
                Key::Escape => {
                    app.quit();
                    std::process::exit(0);
                }
                Key::Up | Key::BackSpace => {
                    if lb.selected_row() == get_visible_row_at_index(&lb, 0) {
                        search.grab_focus();
                        return glib::Propagation::Stop;
                    }
                }
                _ => {}
            }
            glib::Propagation::Proceed
        }
    ));
    controller
}

pub(crate) fn make_search_controller(app: &Application, lb: &ListBox) -> EventControllerKey {
    let controller = EventControllerKey::new();
    controller.connect_key_pressed(clone!(
        #[weak]
        app,
        #[weak]
        lb,
        #[upgrade_or]
        glib::Propagation::Proceed,
        move |_ctrl, key, _, _| {
            match key {
                Key::Escape => {
                    app.quit();
                    std::process::exit(0);
                }
                Key::Return | Key::KP_Enter | Key::ISO_Enter => {
                    select_visible_row_child_at_index(&lb, 0, true, true);
                }
                Key::Down => {
                    select_visible_row_child_at_index(&lb, 1, true, false);
                }
                _ => {}
            }
            glib::Propagation::Proceed
        }
    ));
    controller
}

pub(crate) fn select_visible_row_child_at_index(
    lb: &ListBox,
    i: usize,
    focus: bool,
    activate: bool,
) -> glib::Propagation {
    if let Some(dc) = get_visible_row_at_index(lb, i) {
        if focus {
            dc.grab_focus();
        }
        dc.set_state_flags(StateFlags::SELECTED, false);
        lb.select_row(Some(&dc));

        if activate {
            dc.activate();
        }
    }
    glib::Propagation::Stop
}

pub(crate) fn get_visible_row_at_index(lb: &ListBox, index: usize) -> Option<ListBoxRow> {
    let mut i: i32 = 0;
    let mut count: usize = 0;
    while let Some(child) = lb.row_at_index(i) {
        // For security
        if i > 500 {
            break;
        }
        i += 1;
        let dc = child.downcast_ref::<ListBoxRow>();
        if dc.is_none() {
            eprintln!(
                "Could not downcast to ListBoxRow (keyboard.rs, get_visible_row_at_index function), iteration {i}"
            );
            continue;
        }
        let dc = dc.unwrap();
        if dc.is_visible() && dc.is_mapped() {
            if index == count {
                return Some(dc.clone());
            }
            count += 1;
        }
    }
    None
}
