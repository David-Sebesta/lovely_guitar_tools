# Lovely Guitar Tool

*Still a work in progress*

A Rust-based guitar tool built with `egui` and compiling to WebAssembly.

## Features

-   **Interactive Fretboard**: Visualize notes, scales, and chords on a dynamic fretboard.
-   **Audio Playback (Not Done Yet)**: Simple Web Audio integration for play-testing tones.
-   **Experimental Modes**: Includes modes for reverse scale and chord lookup.
-   **Customizable**: Adjust settings for different tunings and string configurations.

## Prerequisites

1.  **Rust and Cargo**: Ensure you have Rust installed. You can get it from [rustup.rs](https://rustup.rs/).
2.  **Trunk**: This project uses [Trunk](https://trunkrs.dev/) to build and bundle the WASM application.

    Install Trunk via cargo:
    ```sh
    cargo install --locked trunk
    ```

## Running Locally

To start a local development server:

```sh
trunk serve
```

This will build the project and serve it at `http://127.0.0.1:8080` (or another port if 8080 is taken). The server supports hot-reloading.

## Building for Release

To build the project for production (output will be in the `dist` directory):

```sh
trunk build --release
```
