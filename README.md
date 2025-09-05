# PayCalc

A small Rust application to calculate pay with progressive hourly rate increases — now with a GUI built using egui/eframe and persistent settings between sessions.

This README covers:

- what the app does,
- how to run it,
- important UI features (CLI-line parsing, persistence),
- development notes for integrating or extending the calculation logic.

---

## Overview

PayCalc computes payment amounts when an hourly rate increases after a given number of hours (a "step" or "palier"). The app supports:

- a base hourly rate,
- the number of worked hours,
- the increase amount applied after each step,
- the number of hours per step,
- an amount already paid (so the app can show the remaining amount).

The app provides:

- a graphical user interface (egui / eframe),
- the ability to paste a CLI-style line like `*50 40` to quickly set the base rate and hours,
- session persistence (settings and inputs are saved and restored across app restarts),
- a Reset button to restore defaults.

Note: the calculation logic lives in `src/utils.rs`. The GUI calls this logic to compute totals.

---

## Installation

Prerequisites:

- Rust toolchain (cargo)
- A platform supported by eframe/egui

Clone and build:

```bash
git clone https://github.com/ZiedYousfi/PayCalc.git
cd paycalc
cargo build
```

To build an optimized release binary:

```bash
cargo build --release
```

---

## Run

Run the app from the project directory:

```bash
cargo run
```

or the release build:

```bash
./target/release/paycalc
```

When the GUI opens, the left panel contains the inputs:

- `Taux horaire de départ` (base rate in €/h)
- `Heures travaillées (total)` (total worked hours)
- `Augmentation par palier` (+€/h per step)
- `Heures par palier` (hours per step)
- `Montant déjà payé` (€ already paid)
- `Ligne CLI` — paste a CLI-style single line like `*50 40` and click `Appliquer` to parse it into the corresponding fields.

Use:

- `Calculer` to run the payment calculation (it calls `utils::calculate_payment`),
- `Réinitialiser` to reset fields to the defaults,
- `Quitter` to exit the application.

Results (Total earned, Already paid, Remainder) are shown in the main panel.

---

## Persistence

The GUI persists app state between sessions using eframe's persistence feature together with `serde` serialization.

What is persisted:

- Numeric inputs (rate, hours, increase, step hours, already paid)
- CLI line content
- Note text
- `show_advanced` toggle

What is not persisted (transient UI data):

- last displayed summary
- last transient calculation results cached in memory
  These are intentionally marked as transient and will be reset when the app restarts.

Where it's saved:

- eframe writes the settings in a platform-specific application storage location. The precise file or registry location depends on your OS and eframe implementation details.

How to clear persisted settings:

- Use the `Réinitialiser` button to set fields to the defaults, then close the app to let the saved state be overwritten.
- If you need to manually remove saved settings, delete the app-specific storage created by eframe (location is OS-dependent). Alternatively, temporarily change the application name in `main.rs` when running during development so that the app reads/writes a different settings bucket.

Important: The project Cargo.toml already enables eframe's persistence feature and includes `serde` as a dependency:

```toml
eframe = { version="0.32.2", features=["persistence"] }
serde = { version = "1.0", features = ["derive"] }
```

---

## Input format notes

- The CLI-line parser understands strings like `*50 40`:
  - `*50` → hourly rate 50,
  - `40` → worked hours 40.
- The numeric inputs accept decimal values.
- The code uses `utils::get_value` and `utils::get_number_of_hours` for parsing the CLI-style input (see `src/utils.rs`).

---

## Development notes

Project layout:

- `src/main.rs` — starts the eframe app.
- `src/gui.rs` — the egui user interface (inputs, buttons, persistence wiring).
- `src/utils.rs` — CLI helpers and calculation logic (`calculate_payment`, parsers).

Where to put logic:

- The GUI already calls `utils::calculate_payment(...)` in `MyEguiApp::run_calculation`.
- If you want a per-step breakdown instead of a single total, consider modifying `utils::calculate_payment` to return a breakdown structure (e.g. `Vec<(hours, rate, amount)>`) along with the total. Update the GUI to consume and display that breakdown in `gui.rs` (for example in a scrollable list).

Persistence and migrations:

- `MyEguiApp` derives `Serialize`/`Deserialize` (serde). If you add or rename fields later, consider adding versioned migration logic or providing default values via `#[serde(default)]` to avoid deserialization errors on older saved data.

Debugging the UI:

- When iterating on the UI, use `cargo run` or `cargo run --release` and close the app cleanly to ensure changes in UI state are saved (eframe calls the `save` hook at shutdown).
- To avoid reading persistent state during early UI iteration, you can temporarily run with a modified app name in `main.rs` (this isolates storage).

Styling and further UI improvements:

- `gui.rs` already uses `DragValue` widgets for numeric inputs. If you prefer text fields with comma decimal formats or locale-aware formatting, replace those inputs with `TextEdit` and apply parsing/formatting logic in the UI handlers.
- You can add more user guidance, per-step breakdowns, currency formatting, or export results (CSV, JSON) as needed.

---

## Extending the project

Todo ideas:

- Add a detailed breakdown view of amounts per palier.
- Export results into a file (CSV or JSON).
- Add unit tests for `utils::calculate_payment`, `get_value`, and `get_number_of_hours`.
- Add input localization (decimal separators, translations).

---

## License & Author

- License: MIT
- Author: Zied Yousfi
