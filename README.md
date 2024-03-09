# Rust Web Assembly Chrome Extension Example


A sample example using Rust + Leptos in a chrome extension.
This example append a Hello World component on the current page's body element.

Thanks to these repository for their examples :
- [spotify-playlist-builder](https://github.com/rimutaka/spotify-playlist-builder) (details explanation of rust wasm / chrome extension)
- [rust-wasm-chrome-ext](https://github.com/theberrigan/rust-wasm-chrome-ext) (this repo is based on this one)
- [monkeysee](https://github.com/friendlymatthew/monkeysee) (Another leptos chrome extension example)


## Notes
- I tried only with leptos client side rendering.
- Look at the lib.rs which contain most of the logic
- This can be probably simplified and cleaned up.

## Setup
Setup is from [rust-wasm-chrome-ext](https://github.com/theberrigan/rust-wasm-chrome-ext)

1. Install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/):

   ```
   cargo install wasm-pack
   ```
2. Go to ```/wasm_mod``` and run ```build.bat```. <br/>
   It will compile the ```.wasm``` module and ```.js```-wrapper for it and put them in the ```extension/js/wasm```
3. Go to Chrome extensions page and load unpacked extension from ```/extension```

## Notes
- ```extension/js/content.js``` demonstrates how to load wasm into the content script <br/>
  **Important:** in the content script, the module can only be loaded for those sites whose Content Security Policy does not prohibit it
- ```extension/js/background.js``` demonstrates how to load wasm into the background worker script
- For ```wasm-pack``` always use ```--target web```
- ```manifest.json```:
    - To load the wasm module into the content script, you should list ```.wasm``` and corresponding ```.js``` in the ```web_accessible_resources.resources``` section of manifest
    - To load the wasm module into the background worker script, you should specify ```wasm-unsafe-eval``` in the ```content_security_policy.extension_pages``` section of manifest
