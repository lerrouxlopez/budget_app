# Budget App (egui)

A desktop personal budget tracker built with Rust, eframe/egui. It shows planned budget, remaining balance, and recent transactions, with a modal form to add new transactions. Currency labels are in PHP.

## Prerequisites
- Rust toolchain (stable).
- For installer builds: WiX 3.11 binaries. You can download the zip if you cannot install system-wide:  
  `Invoke-WebRequest -Uri https://github.com/wixtoolset/wix3/releases/download/wix3112rtm/wix311-binaries.zip -OutFile wix\wix311-binaries.zip` then  
  `Expand-Archive -Path wix\wix311-binaries.zip -DestinationPath wix\wix311-binaries -Force`

## Running (debug)
```bash
cargo run
```

## Release build
```bash
cargo build --release
```
Binary: `target\release\budget_app.exe`

## MSI installer
Requires the WiX binaries in `wix\wix311-binaries`.
```bash
cargo wix -b wix\wix311-binaries
```
Output: `target\wix\budget_app-0.1.0-x86_64.msi`

## State persistence
Budget and transactions persist to `budget_data.json` in the working directory. Copy this file alongside the executable if you move machines.
