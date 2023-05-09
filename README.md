# Terminal UI Library

A terminal UI library with a custom terminal backend (i.e. no crossterm).

This was made for a terminal-based text editor I am making.

Mouse events are not yet supported and there are currently very few widgets, but most of the hard work has been done (probably famous last words).

Although I've generally tried to write performant code, I don't know how good the performance is as I haven't done any profiling.

You should probably just use [tui-rs](https://github.com/fdehau/tui-rs).

## TODO

- Mouse events
- Testing
- Handle frame timing better.

### Bugs

### Widgets

- Overlap
- Scroll Area
- Button
- Slider
- Checklist

### Currently not planned

- Canvas widget
- Theming
