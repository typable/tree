# tree

A simple tree-viewer CLI
<br>
<br>

### Usage

The following command:
```bash
tree --path=git/tree
```

Results in this output:
```
git/tree/
 ├ Cargo.lock
 ├ Cargo.toml
 ├ LICENSE
 ├ src/
 │  └ main.rs
 └ target/
    ├ CACHEDIR.TAG
    └ debug/
       ├ build/
       ├ deps/
       │  ├ tree.d
       │  ├ tree.exe
       │  └ tree.pdb
       ├ examples/
       ├ incremental/
       ├ tree.d
       ├ tree.exe
       └ tree.pdb
```

**Notice:** If the command is used without any arguments, it will use the current terminal path.
<br>
<br>

### Arguments

All arguments can be use independently with each other.

|Argument|Description|Example|
|---|---|---|
|**--path [-p]**|Specifies the to be used path|`--path=git/tree`|
|**--level [-l]**|Specifies the depth of the tree|`--level=0`|
|**--out [-o]**|Specifies the output file|`--out=tree.txt`|
|**--all [-a]**|Shows all `.*` files and directories|`--all`|
|**--help [-h]**|Shows a list of all commands|`--help`|
