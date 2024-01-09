# Terms

![Matrix](https://img.shields.io/matrix/terms)

A tiling terminal emulator for GNOME on Linux using GTK4, written in Rust.

<!-- <div align="center">

![Main window](data/resources/screenshots/screenshot1.png "Main window")
</div> -->

## Goal

Terms is an experiment. I like [Tilix][Tilix] and I like [BlackBox]. Why can't I have both?

## Status

- [x] Single terminal mode. No tiling, no tabs.
- [x] Multiple windows
- [x] Drag/drop support
- [x] href/mailto regex matching
- [x] Settings
- [x] Theming (should be compatible with Black Box/Tilix)
- [ ] Tabs
- [ ] Tiling
- [ ] Profiles
- [ ] Flatpak support
- [ ] Store and load sessions like Tilix does
- [ ] iTerm2-like support for tmux


<!-- ## Building the project

Make sure you have `flatpak` and `flatpak-builder` installed. Then run the commands below. Replace `io.github.vhdirk.Terms` with the value you entered during project creation. Please note that these commands are just for demonstration purposes. Normally this would be handled by your IDE, such as GNOME Builder or VS Code with the Flatpak extension.

```
flatpak install org.gnome.Sdk//44 org.freedesktop.Sdk.Extension.rust-stable//22.08 org.gnome.Platform//43
flatpak-builder --user flatpak_app build-aux/io.github.vhdirk.Terms.Devel.json
```

## Running the project

Once the project is build, run the command below. Replace Replace `io.github.vhdirk.Terms` and `<project_name>` with the values you entered during project creation. Please note that these commands are just for demonstration purposes. Normally this would be handled by your IDE, such as GNOME Builder or VS Code with the Flatpak extension.

```
flatpak-builder --run flatpak_app build-aux/io.github.vhdirk.Terms.Devel.json <project_name>
``` -->

<!-- ## Community

Join the GNOME and gtk-rs community!
- [Matrix chat](https://matrix.to/#/#rust:gnome.org): chat with other developers using gtk-rs
- [Discourse forum](https://discourse.gnome.org/tag/rust): topics tagged with `rust` on the GNOME forum.
- [GNOME circle](https://circle.gnome.org/): take inspiration from applications and libraries already extending the GNOME ecosystem. -->

## Credits

Lots of ideas and code is ~stolen~ borrowed from these projects:

- [BlackBox]
- [Tilix]
- [Fractal]
- [Zoha]
- [Prompt]

[BlackBox]: https://gitlab.gnome.org/raggesilver/blackbox
[Tilix]: https://github.com/gnunn1/tilix
[Fractal]: https://gitlab.gnome.org/World/fractal
[Zoha]: https://github.com/hkoosha/zoha4
[Prompt]: https://gitlab.gnome.org/chergert/prompt
