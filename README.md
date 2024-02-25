# Define Header (DEHE)

Define Header inserts uuids as `#ifndef` guards in c++ `.h` and `.hpp` files.


## Build
Simply run `cargo build -r`

## Usage

Define Header can target either single files or folders. Simply call the program and supply the path to your folder or file as the first argument.


### Targeting a file  
Suppose you had a header `my-header.h` in your working directory. Call dehe with the file name as argument:

```text
dehe ./my-header.h
```

### Targeting a folder  
dehe will iterate all `.h` and `.hpp` files in the indicated folder
```text
dehe ./folder
```

### Usage Note
dehe will scan the file for lines beginning with `#ifndef u` to determine whether to skip or modify the files