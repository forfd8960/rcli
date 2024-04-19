# RCli

**Rust Cli Tool✌️** ✌️

## Errors

### cargo deny check failed

```
[ERROR] failed to open advisory database: "$HOME/.cargo/advisory-db/github.com-2f857891b7f43c59" does not appear to be a git repository: Could not retrieve metadata of "$HOME/.cargo/advisory-db/github.com-2f857891b7f43c59": No such file or directory (os error 2)
```

fix by adding:

refer: https://github.com/EmbarkStudios/cargo-deny/pull/420

```toml
[advisories]
git-fetch-with-cli = true
```
