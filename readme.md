# Follow [vst3-sys](https://github.com/RustAudio/vst3-sys) for updates to VST3 Bindings 

This project spawned vst3-sys, a Rust crate with raw bindings to the VST3 API. The source here is incomplete and archived. 

# cli-host 

Developed as a debugging/experimenting platform for a port of the VST3 API to Rust. 

---

Usage: 

```
cli-host -p Path/To/Plugin.vst3
```

## Todos/path forward

In addition to the `todos` in the source code;

- [ ] make cross platform, currently only supported on Linux
    - mostly changing some of the paths
- [ ] finish the work in `vst3-host-boilerplate` for creating
      the trait wrappers and Vst3 derives
- [ ] repeat `vst3-host-boilerplate` for the rest of the API
- [ ] split wrapping code into a separate `vst3-sys` crate

## Structure

- `cli-host`
  - proof-of-concept host
- `vst3-impl`, `vst3-derive`
  - macros for creating your own macros,
- `vst3-interfaces`
  - VST3 API ported to Rust

## License 

GPLv3 unless otherwise noted in the source files. The text can be found at [this link](https://www.gnu.org/licenses/gpl-3.0.en.html).

## Credits

The source code in `/vst3-interfaces/macros` has been modified from the [winapi](https://github.com/retep998/winapi-rs/blob/0.3/src/macros.rs) crate
and is licensed under Apache/MIT

`vst3-impl` and `vst3-derive` has been modified from the
[com-impl](https://github.com/Connicpu/com-impl) crate, and is licensed under MIT/Apache
