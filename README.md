## BYFI

command line program to convert binary file to bytes text file or convert byte text file to binary file. and encript decrypt file.

### Build

- `cargo build --release`
- copy and rename `./target/release/byfi` to byfi to your path

### Usage

```
byfi - bytes to file or file to bytes

Usage:
    byfi [COMMAND] [OPTION] FILE_NAME

Command:
    byte                        convert file to bytes text.
    file                        convert bytes text to file.
    en                          encript file
    de                          decript file

Option:
    -b [base]                   ex. `byfi -b 16 file.png` 
                                [base] = 2 | 8 | 16.
                                convert to 2 8 or 16 base.
    --key [string]              to encrypt of decript your file

Example:
    byfi byte file.png          convert `file.png` to bytes text.
                                output is `file.png.txt`.
    byfi file file.png.txt      convert back to file.
```
