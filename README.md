Currently tested on:
macOS
Arch Linux

Installation:

1: Clone repo _git clone 'https://github.com/am1444/sinwave.git'_

2: Go into repo directory.

3: Make sure cargo is installed: _which cargo_ (If cargo is not installed then install Rust then type in 'rustup default stable')

4: _cargo build_. This will produce a binary at ./target/debug/sinwave.


Usage:
_sinwave_

Upon typing in the sinwave command, you will be prompted for the following information:

Rep: The amount of sinwaves to display.

Dist: how many lines each wave should be apart.

Width: terminal width. Ideally this will be detected automatically in a future update.

Wait: Amount of time to sleep in milliseconds between printing each line.
