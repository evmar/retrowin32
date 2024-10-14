Prints function prototypes for Windows API.

Use like:

```
$ cargo run GetDriveTypeA FileTimeToLocalFileTime
```

and it will print the appropriate winapi stub functions.

Requires Windows.Win32.winmd as downloaded from
https://github.com/microsoft/windows-rs/tree/master/crates/libs/bindgen/default
