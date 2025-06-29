> [!IMPORTANT]
> No longer maintained.  If you wish to take over, send a mail to <sebastian@swsnr.de>.

# mdcat

Fancy `cat` for Markdown (that is, [CommonMark][]):

```
$ mdcat sample.md
```

![mdcat showcase with different colour themes][sxs]

mdcat in [WezTerm], with "One Light (base16)", "Gruvbox Light", and "Darcula
(base16)" (from left to right), and [JetBrains Mono] as font.

[CommonMark]: http://commonmark.org
[Solarized]: http://ethanschoonover.com/solarized
[dracula]: https://draculatheme.com/iterm/
[wezterm]: https://wezfurlong.org/wezterm/
[JetBrains Mono]: https://www.jetbrains.com/lp/mono/
[sxs]: ./screenshots/side-by-side.png

## Features

`mdcat` works best with [iTerm2], [WezTerm], and [kitty], and a good terminal font with italic characters.
Then it

* nicely renders all basic CommonMark syntax,
* highlights code blocks with [syntect],
* shows [links][osc8], and also images inline in supported terminals (see above, where "Rust" is a clickable link!),
* adds jump marks for headings in [iTerm2] (jump forwards and backwards with <key>⇧⌘↓</key> and <key>⇧⌘↑</key>),
* displays line numbers with `--line-numbers` for code review and debugging,
* shows document statistics with `--stats` including word count, character count, and reading time estimates.

| Terminal                   |  Basic syntax | Syntax highlighting | Images | Jump marks |
| :------------------------- | :-----------: | :-----------------: | :----: | :--------: |
| Basic ANSI¹                | ✓             | ✓                   |        |            |
| Windows 10 console         | ✓             | ✓                   |        |            |
| [Terminology]              | ✓             | ✓                   | ✓      |            |
| [iTerm2]                   | ✓             | ✓                   | ✓²     | ✓          |
| [kitty]                    | ✓             | ✓                   | ✓²     |            |
| [WezTerm]                  | ✓             | ✓                   | ✓²     |            |
| [VSCode]                   | ✓             | ✓                   | ✓²     |            |
| [Ghostty]                  | ✓             | ✓                   | ✓²     |            |

1) mdcat requires that the terminal supports strikethrough formatting and [inline links][osc8].
    This includes most modern terminal emulators, such as Windows Terminal, KDE Konsole, or anything based on VTE, GNOME's terminal emulation library.
    But mdcat likely won't work well on old terminals that lack these features (e.g. the Linux text console).
2) SVG images are rendered with [resvg], see [SVG support].

Not supported:

* CommonMark extension for footnotes.
* Inline markup and text wrapping in table cells.

[syntect]: https://github.com/trishume/syntect
[osc8]: https://gist.github.com/egmontkob/eb114294efbcd5adb1944c9f3cb5feda
[Terminology]: http://terminolo.gy
[iterm2]: https://www.iterm2.com
[WezTerm]: https://wezfurlong.org/wezterm/
[kitty]: https://sw.kovidgoyal.net/kitty/
[resvg]: https://github.com/RazrFalcon/resvg
[SVG support]: https://github.com/RazrFalcon/resvg#svg-support
[VSCode]: https://code.visualstudio.com/
[Ghostty]: https://mitchellh.com/ghostty

## Usage

Try `mdcat --help` or read the [mdcat(1)](./mdcat.1.adoc) manpage.

## Installation

* [Release binaries](https://github.com/swsnr/mdcat/releases/) built on Github Actions.
  - These binaries are build from Git source on Github actions; you find provenance attestations at <https://github.com/swsnr/mdcat/attestations>.
* 3rd party packages at [Repology](https://repology.org/project/mdcat/versions)
* You can also build `mdcat` manually with `cargo install mdcat` (see below for details).

`mdcat` can be linked or copied to `mdless`; if invoked as `mdless` it automatically uses pagination.

## Building

Run `cargo build --release`.

Building requires `libcurl`.

## Packaging

When packaging `mdcat` you may wish to include the following additional artifacts:

- A symlink or hardlink from `mdless` to `mdcat` (see above).
- Shell completions for relevant shells, by invoking `mdcat --completions` after building, e.g.

  ```console
  $ mdcat --completions fish > /usr/share/fish/vendor_completions.d/mdcat.fish
  $ mdcat --completions bash > /usr/share/bash-completion/completions/mdcat
  $ mdcat --completions zsh > /usr/share/zsh/site-functions/_mdcat
  # Same for mdless if you include it
  $ mdless --completions fish > /usr/share/fish/vendor_completions.d/mdless.fish
  $ mdless --completions bash > /usr/share/bash-completion/completions/mdless
  $ mdless --completions zsh > /usr/share/zsh/site-functions/_mdless
  ```

- A build of the man page `mdcat.1.adoc`, using [AsciiDoctor]:

  ```console
  $ asciidoctor -b manpage -a reproducible -o /usr/share/man/man1/mdcat.1 mdcat.1.adoc
  $ gzip /usr/share/man/man1/mdcat.1
  # If you include a mdless as above, you may also want to support man mdless
  $ ln -s mdcat.1.gz /usr/share/man/man1/mdless.1.gz
  ```

[AsciiDoctor]: https://asciidoctor.org/

## Troubleshooting

`mdcat` can output extensive tracing information when asked to.
Run `mdcat` with `$MDCAT_LOG=trace` for complete tracing information, or with `$MDCAT_LOG=mdcat::render=trace` to trace only rendering.

## License

Copyright Sebastian Wiesner <sebastian@swsnr.de>

Binaries are subject to the terms of the Mozilla Public
License, v. 2.0, see [LICENSE](LICENSE).

Most of the source is subject to the terms of the Mozilla Public
License, v. 2.0, see [LICENSE](LICENSE), unless otherwise noted;
some files are subject to the terms of the Apache 2.0 license,
see <http://www.apache.org/licenses/LICENSE-2.0>
