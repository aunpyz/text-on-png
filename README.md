# text-on-png
## Execute
```
cargo run path/to/image font_size font_family x y width height message path/out
```
`font_size`, `x`, `y`, `width`, and `height` have to be numeric string, otherwise the program will panic
number of arguments have to match exactly 9, or the program will panic
### Example
```
cargo run meme.png 12 sans-serif 44 166 109 150 lorem\ ipsum\ dolor meme_text.png
```
