# CSV to Binary
Converting CSV files to Binary files with ease.

## Installation
You can download the `exe` from the Releases. <i>(if you're on Windows)</i><br>
Or you can build and install manually using `cargo install --path .`

## Usage
Call csv2bin from your command line :
```
$ csv2bin < in.csv > out.csv
```

## Binary Format
<sup>`..=` means an inclusive range :)</sup><br>
All values *(expected to be numbers between `0..=255`)* are converted to bytes <sup>`u8`</sup><br>
Newlines are represented as `0xFF` *(`255`)*
> [!WARNING]  
> Anything that isn't a number is converted to `0`.

<br>

<h2></h2>
<div align="right"><sub>© 2023 Max &lt;mxcop&gt;, All rights reserved — <a href="./license.md">MIT</a>.</sub></div>
