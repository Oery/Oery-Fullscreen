# Oery Fullscreen

A lightweight software to toggle borderless fullscreen on any window.
Only supports Windows.

## Usage

1. Download the latest release from the [releases page](https://github.com/oery/oery-fullscreen/releases).
2. Run the executable.
3. Press `Shift + F11` to toggle fullscreen mode.

To quit the application, click 'Quit' in the tray menu.

## Known issues

-   Windows that aren't meant to be fullscreen, such as Voicemeeter, will break when fullscreen mode is toggled. This can be fixed but as for now, do not toggle fullscreen on these windows.

## Building

1. Install [Rust](https://www.rust-lang.org/tools/install).
2. Clone the repository.
3. Run `cargo build --release`.
4. The executable will be located at `target/release/oery-fullscreen.exe`.
