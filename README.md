# MenuR
A Rust app launching menu, inspired by wofi\
Current Status : Usable\
This menu is made for Wayland on Linux, only with dispatchers implementing the Layershell protocol (such as Hyprland(The one I use, I haven't tested any other)).\

### Dependencies :
- Cargo
- GTK4
- Other things probably

### Building :
With Cargo : 
```shell
cargo build --release
```

### Features :
- Fetch desktop entries : OK
- Get Icons and display them : OK
- Main app window : OK
- Browse trough entries : OK
- Search bar : OK
- start the selected app : OK
- Add shell command execute through the menu: TODO
- Sorting entries : TODO
    - sorting on names : TODO
    - Sorting on cache : TODO
- Add cmdline options : TODO
- Add Configuring : TODO
- Add Theming and CSS : TODO

### More things to do :
- Clean the code
- Optimize entry fetching (cache ?)
- Make it prettier
- Optimize CPU usage (currently not too bad anymore)
