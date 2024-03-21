## BYFI

convert file to bytes text and writes it to text file.

convert binary text back to files.

### Build

- `cargo build --release`
- copy and rename ./target/release/byfi to byfi to your path

### Usage

```
Usage:
    byfi [COMMAND] [OPTION] FILE_NAME

Command:
    byte                        convert file to bytes text.
    file                        convert bytes text to file.

Option:
    -b [base]                   ex. `byfi -b 16 file.png` 
                                [base] = 2 | 8 | 16.
                                convert to 2 8 or 16 base.

Example:
    byfi byte file.png          convert `file.png` to bytes text.
                                output is `file.png.txt`.
    byfi file file.png.txt      convert back to file.
 
```