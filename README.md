# Batch Renamer App

A desktop app written in Rust (GUI built with [egui](https://github.com/emilk/egui)) for renaming many files at once, built by reverse-engineering a more well-known tool that offered a limited free plan.
Add files or folders, combine one or more **rules** (insert, replace, case, serialize, **numbering**, regular expressions, and more), and check the **preview** before renaming. The last rename can be undone, and rule sets can be saved as **presets**. The interface is available in English and Italian.


## Requirements

- **Rust** (recent stable toolchain) with `cargo`
- An internet connection **on the first build only**, to download the dependencies (`eframe 0.27.2`, `rfd`, `regex`, `serde`, `serde_json`, `image`)


### Windows

1. **Install the C++ build tools** (required by the default `msvc` Rust toolchain):
   download [Build Tools for Visual Studio](https://visualstudio.microsoft.com/visual-cpp-build-tools/) and, in the installer, select the **Desktop development with C++** workload.
   Alternatively, with `winget`:
   ```powershell
   winget install Microsoft.VisualStudio.2022.BuildTools --override "--wait --passive --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
   ```
2. **Install Rust** with [rustup](https://rustup.rs/) (download and run `rustup-init.exe`, choose the default installation), or:
   ```powershell
   winget install Rustlang.Rustup
   ```
3. **Close and reopen the terminal**, then verify:
   ```powershell
   rustc --version
   cargo --version
   ```


### Linux (indicative)

You need a C compiler and the system libraries used by `eframe` and `rfd`. On Debian/Ubuntu, for example:

```bash
sudo apt install build-essential pkg-config libxkbcommon-dev libwayland-dev libgtk-3-dev
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Package names may differ on other distributions.


### Build with `build.bat` (Windows)

`build.bat` is a wrapper around `cargo` that adds it to the `PATH` for you, so it also works from a terminal where `cargo` is not available. Run it from the `app/` folder (or double-click it):

```powershell
build.bat                  # optimized release build (same as: cargo build --release)
build.bat run --release    # build and launch
build.bat test             # run the tests
```

Any other arguments are passed straight to `cargo`. The executable ends up in `app\target\release\`.

The script expects Rust in the **default rustup location**, `%USERPROFILE%\.cargo\bin` (for example `C:\Users\<you>\.cargo\bin`). If you installed Rust somewhere else (for instance with a custom `CARGO_HOME`), open `build.bat` and **change the path in the `set TC=...` line** to the `bin` folder of your installation. If `cargo.exe` is not found there, the script stops and says so.


## Build and run

From the `app/` folder:

```powershell
# optimized build
cargo build --release

# build and launch
cargo run --release

# run the tests
cargo test
```


The executable is in `app/target/release/` (`batch-renamer.exe` on Windows). On the first run `cargo` downloads and compiles the dependencies, so it takes a few minutes; later builds are much faster.


## Troubleshooting

| Symptom | Likely cause |
|---|---|
| `linker 'link.exe' not found` | The C++ Build Tools are missing (Windows step 1) |
| `'cargo' is not recognized as a command` | The terminal was not reopened after installing Rust, or the `PATH` is wrong (use `build.bat`, or fix the path in it) |
| `cargo.exe not found in ...` (from `build.bat`) | Rust is not installed in the default location: edit the `set TC=...` line |
| Error while downloading crates | No connection, or a proxy/firewall blocking `crates.io` |
| Changes don't show up in the app | You launched an old executable: rebuild with `cargo build --release` and restart |
