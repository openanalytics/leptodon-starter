# Leptodon starter
This is a template repository to get started with [leptos](https://github.com/leptos-rs/leptos) + [leptodon](https://github.com/openanalytics/leptodon) inside an [axum](https://github.com/tokio-rs/axum) SSR environment.

## Getting Started
1. Click the green `Use this template` button and create a repository using this template.
2. Clone your repository.
3. Rename the project to your liking: `rg -l starter ./ | xargs sed -i 's/starter/yourprojectname/g'` (may need to [install ripgrep](https://github.com/BurntSushi/ripgrep?tab=readme-ov-file#installation)).
4. Install [cargo-leptos](https://github.com/leptos-rs/cargo-leptos): `cargo install --locked cargo-leptos`.
5. [Optional steps](#optional-steps).
6. Run with: `cargo leptos serve`.

The starter should now be available on [http://localhost:3000](http://localhost:3000).

### Optional Steps
1. Run `npm install`.
2. Run Tailwind's CSS generator: `npx tailwindcss -i input.css -o style/output.css --watch`.
    - Tailwind is configured via `tailwind.config.js` and any additional CSS can be added at the end of `input.css`.
4. Setup pre-commit hooks: `pre-commit install` (may need to [install pre-commit](https://pre-commit.com/#installation) first).
    - Pre-commit is configured via [.pre-commit-config.yaml](./.pre-commit-config.yaml).
5. Run `direnv allow` to enable [direnv](https://direnv.com/) in case you use NixOS with flakes.
    - direnv is configured via `.envrc`, `flake.nix` and `flake.lock`.
6. Run `cd end2end; direnv allow` to enable the Playwright devenv, you may need to [install devenv](https://devenv.sh/getting-started/).
    - Run `npm install` (since you also need to install Playwright here).
7. Run `playwright test` to execute the tests.
8. Run `cargo install --locked cargo-make` to enable [Makefile.toml support](https://sagiegurari.github.io/cargo-make/), some useful commands include:
   - Run `cargo make mimic-ci` for a suite of checks your CI would run.
   - Run `cargo make build-starter` to build the project in release mode for production deployment.

## Versions
- Uses Leptos SSR, and based on https://github.com/leptos-rs/start-axum
- Rust stable version 1.92
- Tailwind v3

## What is `build.rs` ?
 - Runs `cargo run codegen` to generate `.tailwind` for Tailwind's class generation.
 - Generates `src/generated_demolist.rs` containing a list of all demos to construct routes and sidebar entries programmatically.
   - Allows for easy alphabetical sorting and reduces maintenance burden of adding/removing entries.
