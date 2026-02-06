# TempLess

A Dark Age of Camelot Template Solver built with [Dioxus](https://github.com/dioxuslabs/dioxus) and [Clingo](https://github.com/potassco/clingo).

# Development

## Clingo
This project uses [clingo](https://github.com/potassco/clingo) as the solver. To bundle clingo into the application, we use a `build.rs` script that compiles clingo from source and links it to the Rust code.

The libraries are compiled depending on the profile (debug or release).
This means
```bash
cargo build --features bundle-clingo
```
and
```bash
cargo build --release --features bundle-clingo
```
will produce debug and release builds of clingo respectively.

## Styling
For styling, we use Tailwind CSS. The `tailwind.css` file contains the Tailwind directives, and we compile it to `assets/tailwind.css` for use in the application.

To compile tailwind, run
```bash
npx tailwindcss -i ./tailwind.css -o ./assets/tailwind.css --watch
```

## Serving
To run the app with hot reloading, use
```bash
dx serve --hot-reload true
```

To run with rust hot patching
```bash
dx serve --hotpatch
```