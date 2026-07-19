# Display a Discord user's avatar from an ID in your terminal

## How to install
```
cargo install terminal-dc-pfp-viewer
```

## Ways to use
### Interactive
```
terminal-dc-pfp-viewer
```

### Non-interactive
```
terminal-dc-pfp-viewer --id 556610358396321840
```

## Notes
### Disclaimer
#### This crate relies on a third-party site, so users can look up avatars **WITHOUT** a **DISCORD TOKEN**.
#### The tool uses the API of a website called https://www.discordpfp.gg. I am not associated with them.
#### The crate **WILL BREAK** if they decide to update their API. It's incredibly easy to update the tool to use another API.
### How to fix in case of breakage / how to adapt it to other API providers
#### You just have to change `src/structs/api_layout.rs` to match the API layout of the third-party site or service you want to use.
#### After that, change the URL found in `src/main.rs` at line 16, and at line 25 change it to match your API layout.
