# Leptodon starter
This is a template repository to get started with [leptos](https://github.com/leptos-rs/leptos) + [leptodon](https://github.com/openanalytics/leptodon) inside an axum ssr environment.

## Getting Started
1. Click the green `Use this template` button and create a repo using this template.
2. Clone your repository
3. Rename to your own project-name: `rg -l starter ./ | xargs sed -i 's/starter/yourprojectname/g'`
4. Install cargo-leptos `cargo install --locked cargo-leptos`
5. [Optional steps](#optional-steps)
6. Run with `cargo leptos serve`

The starter should now be available on http://localhost:3000

### Optional Steps
1. `npm install`
2. Run tailwind's css generator `npx tailwindcss -i input.css -o style/output.css --watch`.
    - Tailwind is configured via `tailwind.config.js` and manual css can go at the end off `input.css`.
4. Setup commit hooks `pre-commit install`, may need to install pre-commit first.
    - Pre-commit is configured via [.pre-commit-config.yaml](./.pre-commit-config.yaml).
5. `direnv allow` to enable the direnv in case you use nixos with flakes.
    - The direnv is configured via `.envrc`, `flake.nix` and `flake.lock`.
6. `cd end2end; direnv allow` to enable the playwright devenv, you may need to install `devenv`.
    - `npm install`. You also need to install playwright here.
7. `playwright test` to run the tests.
8. `cargo install --locked cargo-make` to enable Makefile.toml support, some useful commands include:
   - `cargo make mimic-ci` to run a suite of checks your CI would run.
   - `cargo make build-starter` build the project in release mode for deployment.

## Versions
- Uses leptos ssr, and based on https://github.com/leptos-rs/start-axum
- Rust stable version 1.92
- Tailwindv3

## `build.rs` ?
 - Runs `cargo run codegen` to generate `.tailwind` for tailwind's class generation.
 - Generates `src/generated_demolist.rs` containing a list of all demos to construct routes and sidebar entries programatically.
   - Allows for easy alfabetical sorting and reduces maintenance burden of adding/removing entries.
