# Command-Line Help for `pointer-chase`

This document contains the help content for the `pointer-chase` command-line program.

**Command Overview:**

* [`pointer-chase`↴](#pointer-chase)

## `pointer-chase`

Generates a random cycle of pointers and walks it in a loop, recording the time per pointer access

**Usage:** `pointer-chase [OPTIONS] --size <SIZE>`

###### **Options:**

* `-s`, `--size <SIZE>` — Size of the pointer cycle in bytes. Will be rounded down to next multiple of pointer width
* `-t`, `--time <TIME>` — Duration in milliseconds

  Default value: `1000`
* `-e`, `--exit <EXIT>` — Exit after printing set number of measurements
* `--print-header` — Print header

  Possible values: `true`, `false`

* `--markdown-help`

  Possible values: `true`, `false`




<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

