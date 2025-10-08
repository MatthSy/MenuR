// Code from https://docs.rs/wayland-client/latest/wayland_client/globals/index.html#example mostly
use crate::wl_wrap::smt_tk::globals::GlobalData;
use smithay_client_toolkit::{self as smt_tk, shell::wlr_layer::LayerShellHandler};
use smt_tk::reexports::client::{
    globals::{registry_queue_init, GlobalList, GlobalListContents},
    protocol::{/* wl_compositor,*/ wl_registry},
    Connection, Dispatch, EventQueue, QueueHandle,
};
use smt_tk::reexports::protocols_wlr::layer_shell::v1::client::{
    zwlr_layer_shell_v1::ZwlrLayerShellV1, zwlr_layer_surface_v1::ZwlrLayerSurfaceV1,
};
use smt_tk::shell::wlr_layer::{LayerShell, LayerSurfaceData};

pub(crate) struct State;

impl Dispatch<wl_registry::WlRegistry, GlobalListContents> for State {
    fn event(
        _state: &mut State,
        _proxy: &wl_registry::WlRegistry,
        _event: wl_registry::Event,
        // This mutex contains an up-to-date list of the currently known globals
        // including the one that was just added or destroyed
        _data: &GlobalListContents,
        _conn: &Connection,
        _qhandle: &QueueHandle<State>,
    ) {
        /* react to dynamic global events here */
    }
}
impl Dispatch<ZwlrLayerShellV1, GlobalData> for State {
    fn event(
        _state: &mut Self,
        _proxy: &ZwlrLayerShellV1,
        _event: <ZwlrLayerShellV1 as smithay_client_toolkit::reexports::client::Proxy>::Event,
        _data: &GlobalData,
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
        todo!()
    }
}

impl Dispatch<ZwlrLayerSurfaceV1, LayerSurfaceData> for State {
    fn event(
        _state: &mut Self,
        _proxy: &ZwlrLayerSurfaceV1,
        _event: <ZwlrLayerSurfaceV1 as gdk4_wayland::wayland_client::Proxy>::Event,
        _data: &LayerSurfaceData,
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
        // todo!()
    }
}

impl LayerShellHandler for State {
    fn closed(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _layer: &smithay_client_toolkit::shell::wlr_layer::LayerSurface,
    ) {
        todo!()
    }

    fn configure(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _layer: &smithay_client_toolkit::shell::wlr_layer::LayerSurface,
        _configure: smithay_client_toolkit::shell::wlr_layer::LayerSurfaceConfigure,
        _serial: u32,
    ) {
        todo!()
    }
}

pub(crate) fn get_globals_and_queue() -> (GlobalList, EventQueue<State>) {
    let conn = Connection::connect_to_env().expect("No connection to the compositor");
    registry_queue_init(&conn).unwrap()
}

#[allow(dead_code)]
pub(crate) fn get_globals() -> GlobalList {
    let conn = Connection::connect_to_env().unwrap();
    let (globals, _) = registry_queue_init::<State>(&conn).unwrap();
    globals
}

pub(crate) fn get_layershell(
) -> Result<LayerShell, smithay_client_toolkit::reexports::client::globals::BindError> {
    let (globals, queue) = get_globals_and_queue();
    let qhandle = queue.handle();

    LayerShell::bind(&globals, &qhandle)
}

// now you can bind the globals you need for your app
// let compositor: wl_compositor::WlCompositor = globals.bind(&queue.handle(), 4..=5, ()).unwrap();
