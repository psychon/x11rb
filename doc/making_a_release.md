```
vim doc/changelog.md  # Update the changelog
vim Cargo.toml  # Update version number
# Now get these changes merged
cargo publish --dry-run
git tag -a -m "Version 0.2.0" v0.2.0
cargo publish
git push origin v0.2.0
```
