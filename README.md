# componenthost
Minimal example of wasmtime about generating the wit for a component.

```bash
cargo build
```

The result of the command above is the following

```bash
error: expected `default`, `world` or `interface`, found an identifier
            --> /wit/world.wit:3:1
             |
           3 | package local:demo
             | ^------
 --> src/main.rs:1:1
  |
1 | wasmtime::component::bindgen!("world" in "wit/world.wit");
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: this error originates in the macro `wasmtime::component::bindgen` (in Nightly builds, run with -Z macro-backtrace for more info)
```