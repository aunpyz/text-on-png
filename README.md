# text-on-png
## Execute
```
cargo run path/to/image font_size font_family x y width height message path/out
```
`font_size`, `x`, `y`, `width`, and `height` have to be numeric string, otherwise the program will panic
number of arguments have to match exactly 9, or the program will panic
### Example
```
cargo run meme.png 12 sans-serif 44 166 109 150 "Lorem ipsum dolor sit amet, consectetur adipiscing elit." meme_text.png
```
### Result
![image](https://user-images.githubusercontent.com/14342782/91122150-c9fc7c80-e6c3-11ea-95f1-d6e5967dd336.png)
