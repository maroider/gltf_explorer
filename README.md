# Development

## Build local documentation for dependencies

`cargo doc --no-deps --document-private-items --open -p gltf-explorer -p clap -p gltf -p gltf-json -p iced -p iced_core -p iced_futures -p iced_graphics -p iced_native -p iced_style -p iced_wgpu -p iced_winit -p native-dialog -p wgpu -p winit`

# Crates used
| name            | purpose                                                              | license        | link                                                 |
| --------------- | -------------------------------------------------------------------- | -------------- | ---------------------------------------------------- |
| `chrono`        | Nicely formatted dates                                               | MIT/Apache 2.0 | [link](https://github.com/chronotope/chrono)         |
| `clap`          | Command-line argument parsing                                        | MIT/Apache 2.0 | [link](https://github.com/clap-rs/clap)              |
| `fern`          | Consuming `log` log records                                          | MIT            | [link](https://github.com/daboross/fern)             |
| `gltf`          | Parsing and consuming glTF documents                                 | MIT/Apache 2.0 | [link](https://github.com/gltf-rs/gltf)              |
| `iced`          | GUI                                                                  | MIT            | [link](https://github.com/hecrj/iced)                |
| `log`           | Logging                                                              | MIT/Apache 2.0 | [link](https://github.com/rust-lang/log)             |
| `native-dialog` | Native file dialogs                                                  | MIT            | [link](https://github.com/balthild/native-dialog-rs) |
