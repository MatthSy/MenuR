use gtk4::glib::{self, clone};
use gtk4::prelude::{ApplicationExt, Cast, WidgetExt};
use gtk4::{gdk::Key, Application, EventControllerKey, ListBox};
use gtk4::{ListBoxRow, StateFlags};

pub(crate) fn make_window_controller(app: &Application, lb: &ListBox) -> EventControllerKey {
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
                    select_first_visible_box_row_child(&lb, true, true);
                }
                Key::Down => {
                    select_first_visible_box_row_child(&lb, true, false);
                }
                _ => {}
            }
            glib::Propagation::Proceed
        }
    ));
    controller
}

pub(crate) fn select_first_visible_box_row_child(
    lb: &ListBox,
    focus: bool,
    activate: bool,
) -> glib::Propagation {
    if let Some(dc) = get_first_visible_row(lb) {
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

pub(crate) fn get_first_visible_row(lb: &ListBox) -> Option<ListBoxRow> {
    let mut i: i32 = 0;
    while let Some(child) = lb.row_at_index(i) {
        // For security
        if i > 500 {
            break;
        }
        i += 1;
        let dc = child.downcast_ref::<ListBoxRow>();
        if dc.is_none() {
            eprintln!(
                "Could not downcast to ListBoxRow (keyboard.rs, get_first_visible_row function), iteration {i}"
            );
            continue;
        }
        let dc = dc.unwrap();
        if dc.is_visible() && dc.is_mapped() {
            return Some(dc.clone());
        }
    }
    None
}
