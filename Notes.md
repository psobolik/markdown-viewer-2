# Markdown Viewer Notes

Jun 2025

This is a Tauri app that uses Leptos for the frontend. Leptos generates web files from Rust
code, and takes the place of the TypeScript/React code we've used in other Tauri apps. 

The program is built with the command
```shell
  cargo tauri build
```
The `Cargo.toml` file in the project's root specifies `src-tauri` as a workspace, where 
`tauri-build` is a build dependency. `tauri-build` uses the contents of `tauri.conf.json` 
to build the app. Before building the code in `src-tauri`, this command runs `trunk build` 
to generates web files from the files in `src`, and places them in `./dist`. Then it builds 
the Tauri executable, which incorporates the generated web code, and places the results in 
`./target/release`. At last it generates bundles for installing the program in 
`./target/release/bundle`.