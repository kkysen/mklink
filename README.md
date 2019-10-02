# mklink

This is primarily a way to easily call `mklink` from WSL. 
Linux symlinks created in WSL are incompatible with Windows, 
but Windows symlinks and directory junctions are compatible with WSL, 
so creating symlinks through `mklink` is preferable.

`mklink` has to be called through `cmd.exe`, however, 
and doesn't appear to be a standalone binary. When I tried to call `cmd.exe /C mklink [...args]` 
from WSL, it worked in bash but not programmatically, 
so I'm creating a better solution here instead.

This Rust project, when compiled for Windows, 
creates a Windows executable, `mklink.exe`, 
that recreates the functionality of `mklink` in `cmd.exe`, 
just in a standalone binary, by calling the Win32 APIs directly.

When compiled for Linux (assuming WSL), it creates a `mklink` binary 
that converts WSL paths to Windows paths and then `exec`s `mklink.exe`.

## Usage
```mklink 0.1.0
 
 USAGE:
     mklink [FLAGS] <target> <link>
 
 FLAGS:
     -d, --dir
     -f, --file
     -h, --hard
         --help       Prints help information
     -V, --version    Prints version information
 
 ARGS:
     <target>
     <link>
```

## Installation
You need to clone the repo 
and then run `cargo install --path [BUILD PATH]` 
both from Windows (PowerShell) and from WSL. 
To do this, you need to have Rust installed both on Windows and WSL.
You could also try cross-compiling, but I think this way is easier.
