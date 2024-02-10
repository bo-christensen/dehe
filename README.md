# Define Header (DEHE)
This is a very tiny CLI tool meant to help define `#IFNDEF <identifier>` guards in c++ .h and .hpp files.

All it does is generate a uuid and modify the specified file adding a header guard using #IFNDEF <uuid> as the identifier for the guard.

The use if UUID is to avoid needing to change the guard name if the header artifacts change.

## Build
Simply run `cargo build -r`

## Usage
dehe `path-to-file`

**Example**

```
dehe /src/file.hpp
```

## Warning
I haven't yet had the tool mangle any of my files, but I also didn't bother writing any tests to ensure stability, as you may be able to tell as the entire tool is a scant 50 lines of code or so.
