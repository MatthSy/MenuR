use gtk4::glib::{self, clone};
use gtk4::prelude::{ApplicationExt, Cast, WidgetExt};
use gtk4::ListBoxRow;
use gtk4::{gdk::Key, Application, EventControllerKey, ListBox};

pub(crate) fn make_window_controller(app: &Application) -> EventControllerKey {
    let controller = EventControllerKey::new();
    controller.connect_key_pressed(clone!(
        #[weak]
        app,
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
                    println!("Activateedededed");
                    select_first_visible_box_row_child(&lb, true);
                }
                Key::Down => {
                    select_first_visible_box_row_child(&lb, false);
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
    activate: bool,
) -> glib::Propagation {
    let mut i: i32 = 0;
    while let Some(child) = lb.row_at_index(i) {
        // For security
        if i > 500 {
            break;
        }
        i += 1;
        let dc = child.downcast_ref::<ListBoxRow>();
        if dc.is_none() {
            eprintln!("Could not downcast to ListBoxRow (keyboard.rs), iteration {i}");
            continue;
        }
        let dc = dc.unwrap();
        if dc.is_visible() && dc.is_mapped() {
            dc.grab_focus();
            dc.set_state_flags(gtk4::StateFlags::SELECTED, true);

            if activate {
                dc.activate();
            }
            break;
        }
    }
    glib::Propagation::Stop
}
