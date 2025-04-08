# Termina

[![Crates.io](https://img.shields.io/crates/v/termina.svg)](https://crates.io/crates/termina)
[![Documentation](https://docs.rs/termina/badge.svg)](https://docs.rs/termina)

A cross-platform "virtual terminal" (VT) manipulation library.

Termina only "speaks text/VT" but aims to work on Windows as well as *NIX. This is made possible by Microsoft's investment into [ConPTY](https://devblogs.microsoft.com/commandline/windows-command-line-introducing-the-windows-pseudo-console-conpty/). This means that Termina requires 64-bit Windows 10.0.17763 (released around Fall 2018) or later ([same as WezTerm](https://wezterm.org/install/windows.html)).

Termina is a cross between [Crossterm](https://github.com/crossterm-rs/crossterm) and [TermWiz](https://github.com/wezterm/wezterm/blob/a87358516004a652ad840bc1661bdf65ffc89b43/termwiz/README.md) with a lower level API which exposes escape codes to consuming applications. The aim is to scale well in the long run as terminals introduce VT extensions like the [Kitty Keyboard Protocol](https://sw.kovidgoyal.net/kitty/keyboard-protocol/) or [Contour's Dark/Light mode detection](https://contour-terminal.org/vt-extensions/color-palette-update-notifications/) - those requiring minimal changes in Termina and also allowing flexibility in how applications detect and handle these extensions. See `examples/event-read.rs` for a look at a basic API.

## Credit

Termina contains significant code sourced and/or modified from other projects, especially Crossterm and TermWiz. See "CREDIT" comments in the source for details on what was copied and what modifications were made. Since all copied code is licensed under MIT, Termina is offered under the MIT license as well at your option.

<details><summary>Crossterm license...</summary>

```
MIT License

Copyright (c) 2019 Timon

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

</details>

<details><summary>TermWiz license...</summary>

```
MIT License

Copyright (c) 2018 Wez Furlong

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

</details>

## License

Licensed under either of:

 * Mozilla Public License, v. 2.0, ([LICENSE-MPL](./LICENSE-MPL) or http://mozilla.org/MPL/2.0/)
 * MIT license ([LICENSE-MIT](./LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.
