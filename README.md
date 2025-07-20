# PyAMA Rust - Development

This project is a Rust implementation of PyAMA (Python Automated Microscopy Analysis) using the Dioxus framework for the desktop GUI.

## Project Structure

```
pyama-rust/
├─ assets/ # Static assets including CSS and images
│  ├─ favicon.ico
│  ├─ header.svg
│  ├─ tailwind.css
│  └─ styling/
│     └─ main.css
├─ prototype/ # HTML prototype for UI design
│  └─ index.html
├─ src/
│  ├─ main.rs # App entrypoint and route definitions
│  ├─ components/ # Reusable UI components
│  │  ├─ mod.rs # Component module definition and exports
│  │  ├─ types.rs # Common types and enums
│  │  ├─ pattern_cell.rs # Grid cell component for micropatterns
│  │  ├─ detail_panel.rs # Pattern information display
│  │  ├─ viewer_pane.rs # Main pattern viewing interface
│  │  ├─ traces_pane.rs # Trace analysis interface
│  │  ├─ top_bar.rs # Application top navigation
│  │  └─ sidebar.rs # Side navigation panel
│  └─ views/ # Route-specific view components
│     ├─ mod.rs # View module definition
│     ├─ welcome_screen.rs # Initial app screen
│     └─ main_app.rs # Main application interface
├─ Cargo.toml # Project dependencies and configuration
└─ Dioxus.toml # Dioxus-specific configuration
```

### Tailwind
1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the Tailwind CSS CLI: https://tailwindcss.com/docs/installation
3. Run the following command in the root of the project to start the Tailwind CSS compiler:

```bash
npx tailwindcss -i ./tailwind.css -o ./assets/tailwind.css --watch
```

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve --platform desktop
```

To run for a different platform, use the `--platform platform` flag. E.g.
```bash
dx serve --platform desktop
```


