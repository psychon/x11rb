```
vim doc/changelog.md  # Update the changelog
vim {x11rb,x11rb-async,x11rb-protocol}/Cargo.toml  # Update version number in all published crates
vim x11rb/Cargo.toml  # Update version number in x11rb-protocol dependency
vim x11rb-async/Cargo.toml  # Update version number in x11rb and x11rb-protocol dependency
# Now get these changes merged
git tag -a -m "Version 0.2.0" v0.2.0
cargo publish --dry-run -p x11rb-protocol
cargo publish -p x11rb-protocol
cargo publish --dry-run -p x11rb
cargo publish -p x11rb
cargo publish --dry-run -p x11rb-async
cargo publish -p x11rb-async
git push origin v0.2.0
```
