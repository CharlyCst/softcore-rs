# Developer Documentation

## Git Cliff

We use [`git-cliff`](https://git-cliff.org/docs/) to generate changelogs from the git history.

- **Update changelog**: `git-cliff -o CHANGELOG.md --tag vX.X.X`

## Publishing a new version

Checklist:

- [ ] Update version numbers
- [ ] Generate changelog
- [ ] Create tag and Github release
- [ ] Publish to Crate.io

To publish to Crate.io:

- First publish the prelude: `cargo publish --package softcore-prelude --dry-run`
- Then publish the cores: `cargo publish --package softcore-rv64 --dry-run`
