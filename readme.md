# CSV to Binary
Converting CSV files to Binary files with ease.

## Binary Format
<sup>`..=` means an inclusive range :)</sup><br>
All values *(expected to be numbers between `0..=255`)* are converted to bytes <sup>`u8`</sup><br>
Newlines are represented as `0xFF` *(`255`)*
> [!WARNING]  
> Anything that isn't a number is converted to `0`.

<br>

<h2></h2>
<div align="right"><sub>© 2023 Max &lt;mxcop&gt;, All rights reserved — <a href="./license.md">MIT</a>.</sub></div>
