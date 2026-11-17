# nsh-input-repro

Minimal repro for an NSH macOS bug: a single-line input's text moves sideways
when the input takes the focus, and by how much depends on which input was
focused before.

NSH: `longbridge/nsh` at `9042f3be` (main, 2026-11-12), pinned in
`.cargo/config.toml`. macOS 27.

```sh
cargo run                  # the first long input is focused at launch
REPRO=switch cargo run     # the focus moves between the short input and the first long one every 1.5s
```

The window shows the same long text (wider than its field) in two inputs, the
first one focusable, and a short input. Compare the first input's text with
the never-focused copy under it:

| Case | Focused text vs unfocused |
| --- | --- |
| Long input focused at launch | 5pt to the left |
| Long input focused after the short one | 2pt to the left |
| Short input | aligned |

See `src/lib.rs` for the whole app.
