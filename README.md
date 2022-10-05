# Lunula

A window manager written in rust that rips off bspwm.
Don't use this software and I will not provide any support if you do. Feel free to use the code as a reference though.

Built using xcb: https://docs.rs/xcb/latest/xcb/

## Features
Floatings windows. Can be resized and moved. ALT is used as the mod key.


## IPC Commands
Commands are made up of a command and an option.
Variables all start with an &.

```sh
focus-workspace &workspace-right
focus-workspace &workspace-left
focus-workspace 1


kill-window &window-selected
```