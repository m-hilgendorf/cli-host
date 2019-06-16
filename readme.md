# cli-host 

Hosts a VST3 plugin by loading it from the command line. 

Developed as a debugging/experimenting platform for a port of the VST3 API to Rust. 

---

Usage: 

```
cli-host -p Path/To/Plugin.vst3
```

## License 

GPLv3, except for the files `macros.rs` and `shared.rs` which are under the Apache License
and originate from the [winapi](https://crates.io/crates/winapi) crate. 

