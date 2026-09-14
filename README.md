# terminal-dc-pfp-viewer

Display a Discord user's avatar in your terminal from a Discord user ID.

## What it does

- Accepts a Discord user ID from `--id` or an interactive prompt
- Looks up avatar data through `https://www.discordpfp.gg/api/discordlookup`
- Downloads the full-size avatar image
- Clears the screen and renders the image directly in your terminal

No Discord token is required.

## Install

```bash
cargo install terminal-dc-pfp-viewer
```

Or build locally:

```bash
cargo build
```

## Usage

Interactive mode (prompts for ID):

```bash
terminal-dc-pfp-viewer
```

Non-interactive mode:

```bash
terminal-dc-pfp-viewer --id 556610358396321840
```

Show CLI help:

```bash
terminal-dc-pfp-viewer --help
```

## Current project layout

```text
src/
├── main.rs                # CLI entrypoint, API call, image download/render flow
├── structs.rs             # module declarations
└── structs/
    ├── cli.rs             # clap CLI definition (`--id`)
    └── api_layout.rs      # serde models for API response parsing
```

## Third-party API dependency

This project currently depends on `discordpfp.gg`'s API schema and endpoint. If their API changes, update:

- `src/structs/api_layout.rs` (response schema)
- `src/main.rs` (endpoint and/or response field usage)

I am not affiliated with `discordpfp.gg`.
