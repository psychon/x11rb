```
vim doc/generated_code.md  # Update the generated code
vim doc/changelog.md  # Update the changelog
vim Cargo.toml  # Update version number
# Now get these changes merged
cargo publish --dry-run
git tag -a -m "Version 0.2.0" v0.2.0
cargo publish
git push origin v0.2.0
```
TODO: Update the above for the split with the new x11rb-protocol crate.
Both crate versions should be kept close together. Still, if there are no
semver-incompatible changes in x11rb-protocol, only the micro version needs a
bump.

Also: Beware! The version for the x11rb -> x11rb-protocol dependency needs to be
properly updated on version changes!
