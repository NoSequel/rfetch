<img src="https://raw.githubusercontent.com/NoSequel/rfetch/main/screenshots/screenshot1.png" width="40%" align="right">
<h1 align="center">rfetch</h1>
<p align="center">is a system information tool written in Rust</p><br>

``rfetch`` is a rewrite of [efetch](https://github.com/NoSequel/efetch) by me to learn more about the Rust programming language, and to improve the terrible performance/structure of efetch. Currently support for multiple operating systems is lacking, however it is pretty easy to add more operating systems. If you wish, you can open a [PR](https://github.com/NoSequel/rfetch/pulls) to add more support and/or clean up code.

## OS Support
Currently we support 6 OS' (3 untested).
* Linux:
    - Gentoo/Linux (without emerge support)
    - Debian/Linux (dpkg)
    - Arch/Linux (pacman)
    - Void/Linux (xbps)

## Configuration
There is a ``config.rs`` file included with the source code, this file is a lot more complex to configure than [efetch's](https://github.com/NoSequel/efethch/src/config.h), but it is way more versatile, allowing to add/remove certain fields, easily add/remove operating systems and package managers, and more.

## Installation
    # 1. Compiling
    We use Rust's package manager Cargo to compile this project, simply type: ``cargo build --release`` to build it.

    # 2. Making a symlink
    It is required to make a symlink to easily access rfetch, simply type ``ln target/release/rfetch /bin/rfetch``.

    # 3. Complete!
    The setup process should now be complete, simply type ``rfetch`` in your personal favorite shell, and it should output the system information!

## Benchmark Results
Although speed doesn't mean a lot in a simple system information fetch program, I've benchmarked this against 3 other system information fetch programs, including [neofetch](https://github.com/dylanaraps/neofetch), [pfetch](https://github.com/dylanaraps/pfetch), and [efetch](https://github.com/NoSequel/efetch), results:
<img src="https://raw.githubusercontent.com/NoSequel/rfetch/main/screenshots/screenshot2.png" width="100%" align="right">
