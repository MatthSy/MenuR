mod entries;
mod keywords;
mod ui;
mod wl_wrap;

use ui::ui;

use gtk4::{Application, glib, prelude::*};
use std::env;
use std::os::unix::process::CommandExt;

const APP_ID: &str = "org.gtk_rs.menu_gtk_1";

#[cfg(target_os = "linux")]
fn main() -> glib::ExitCode {
    // If env variables are set correctly : will run the apps
    if let Ok(key1) = env::var("GSK_RENDERER")
        && key1 == "cairo"
    // && let Ok(key2) = env::var("GDK_DISABLE")
    // && key2 == "gl"
    {
        let app = Application::builder().application_id(APP_ID).build();
        app.connect_activate(ui);

        return app.run(); // Runs the app
    }

    // If they are not set, it wil set them and recall the exe
    // This adds probably over 10ms to the start time
    println!("Env variables were not set, restarting...");
    unsafe {
        env::set_var("GSK_RENDERER", "cairo"); // force CPU renderer (Real impact on start up time)
        // env::set_var("GDK_DISABLE", "gl"); // disable OpenGL (small impact)
    }
    let err = std::process::Command::new(env::current_exe().unwrap())
        .args(env::args().skip(1)) // Gives the current args to the next execution
        .envs(env::vars())
        .exec();

    eprintln!("Failed to re-exec: {:?}", err);
    std::process::exit(1);

    // NOTE: Remove later:
    //
    // unsafe {
    //     env::set_var("GSK_RENDERER", "cairo"); // force CPU renderer (Real impact on performance)
    //     env::set_var("GDK_DISABLE", "gl"); // disable OpenGL (small impact)
    // }
    // let app = Application::builder().application_id(APP_ID).build();
    // app.connect_activate(ui);
    //
    // return app.run();
}
