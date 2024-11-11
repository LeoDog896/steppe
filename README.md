# steppe

tauri + xterm powered terminal emulator focused on being customizable and pretty!

base code from https://github.com/marc2332/tauri-terminal

## ideology

(i might make this a blogpost)

i do NOT care about terminal performance!!! look. i like fast terminal emulators.
i think they're cool and awesome and super cool!! i think rotting software sucks and we should
be building faster software from the ground up! but. i would like my terminal to be mine!
i want more than realizing my [fast:tm: terminal emulator doesn't support ligatures](https://github.com/alacritty/alacritty/issues/50)
or noticing that kitty doesn't have [tmux support](https://gavinhoward.com/2022/02/goodbye-kitty/), and not
being able to do anything about either because its hard to modify the source code!

i want a terminal thats *easy to modify* and one that you can do *silly things with*! but i would
also like my terminal to be practical. theres a few things i would like to add to this emulator that i havent done yet:

## todos

### necessities

- refactor, comment, and clean up codebase: *i want this terminal to be modifiable at source*. plugins would be nice too!
- choose launch shell
- terminal tabs
- terminator-like pane splitting
- an actual settings file

### niceties

- window vibrancy (blurred, translucent background!)
- terminal pets and general display niceities
- confetti
- warp.cli-like terminal blocks (and easy asciicinema-based sharing of terminal blocks)
