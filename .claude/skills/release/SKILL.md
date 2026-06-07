---
name: release
description: Cut a new leptosbook release — bump the version, update CHANGELOG, verify, tag, and publish to crates.io via the GitHub release workflow. Use when the user asks to "release", "cut a release", "publish to crates.io", "ship vX.Y.Z", or bump the crate version.
---

# Releasing leptosbook

This skill cuts a versioned release of the `leptosbook` crate. The actual
`cargo publish` is performed by the `.github/workflows/publish.yml` workflow,
which fires when a GitHub Release is published. This skill's job is to get the
repo into a correct, verified, tagged state and create that release.

**Publishing to crates.io is irreversible** — a version can be *yanked* but never
re-used or deleted. Treat the tagging/release step as a one-way door and confirm
with the user before taking it.

## Inputs

- Target version comes from the argument (e.g. `/release 0.2.0`).
- If no argument is given, use the `version` already in `Cargo.toml` (this is the
  normal path for the very first `0.1.0` release).
- The version MUST be valid semver and MUST be greater than the latest tag.

## Preconditions — verify all before changing anything

Run these and STOP with a clear message if any fails:

1. Working directory is the leptosbook repo root (`Cargo.toml` has `name = "leptosbook"`).
2. On the `main` branch: `git branch --show-current` → `main`.
3. Clean working tree: `git status --porcelain` is empty.
4. Up to date with origin: `git fetch` then confirm `main` is not behind.
5. The tag `vX.Y.Z` does not already exist: `git tag -l vX.Y.Z` is empty.
6. The crates.io publish token secret exists, or the workflow will fail:
   `gh secret list` includes `CARGO_REGISTRY_TOKEN`. If it is missing, tell the
   user to add it (https://crates.io/settings/tokens →
   `gh secret set CARGO_REGISTRY_TOKEN`) and STOP — do not tag.
7. For the **first** release only: confirm the name is available/owned on
   crates.io (`curl -s -o /dev/null -w '%{http_code}' -A 'release (email)' https://crates.io/api/v1/crates/leptosbook` — 404 = free to claim, 200 = already exists; if it exists and isn't yours, STOP).

## Steps

### 1. Set the version

Ensure `Cargo.toml` `version = "X.Y.Z"` matches the target. Edit it if bumping.

### 2. Update CHANGELOG.md

- Confirm there is a `## [X.Y.Z] - <today YYYY-MM-DD>` heading. If the work is
  still under `## [Unreleased]`, rename that heading to the version + today's
  date and insert a fresh empty `## [Unreleased]` above it.
- Update the link refs at the bottom:
  - `[Unreleased]: .../compare/vX.Y.Z...HEAD`
  - `[X.Y.Z]: .../releases/tag/vX.Y.Z`
- The release notes for the GitHub release are the body of this version's
  section — capture that text now (everything between this version's heading and
  the next `##` heading).

### 3. Verify (all must pass — do not continue on failure)

```bash
cargo fmt --all -- --check
cargo test -p leptosbook
cargo clippy -p leptosbook --no-default-features -- -D warnings
cargo clippy -p leptosbook --features ssr -- -D warnings
cargo clippy -p leptosbook --features hydrate -- -D warnings
cargo build -p basic-folio -p gesture-demo -p onboarding-tour
cargo publish -p leptosbook --dry-run
```

`cargo publish --dry-run` is the critical gate — it packages exactly what
crates.io will receive. Also skim `cargo package -p leptosbook --list` to make
sure no junk (or, worse, secrets) is bundled and that `examples/`, `cookbook/`,
and `.github/` are excluded per `Cargo.toml`'s `exclude`.

### 4. Commit and tag

```bash
git add -A
git commit -m "Release vX.Y.Z"
git tag -a vX.Y.Z -m "vX.Y.Z"
```

Use the commit trailer `Co-Authored-By: Claude Opus 4.8 <noreply@anthropic.com>`.

### 5. Confirm with the user, then push and release

Show the user the version, the changelog notes, and that this will publish to
crates.io. On confirmation:

```bash
git push origin main --follow-tags
gh release create vX.Y.Z --title "vX.Y.Z" --notes "<this version's changelog section>"
```

Creating the release triggers `publish.yml`.

### 6. Verify the publish

- Watch the workflow: `gh run watch <id> --exit-status` (find it with
  `gh run list --workflow Publish`).
- Confirm the version is live: poll
  `curl -s https://crates.io/api/v1/crates/leptosbook | grep -o '"max_version":"[^"]*"'`
  until it shows `X.Y.Z` (indexing can lag a minute or two).
- Report the crates.io URL, the docs.rs URL (https://docs.rs/leptosbook/X.Y.Z),
  and the GitHub release URL.

## If the publish workflow fails

Common causes:
- `CARGO_REGISTRY_TOKEN` missing/expired → fix the secret, then re-run the
  workflow (`gh run rerun <id>`); no new tag needed.
- Version already published → bump to the next patch and start over (crates.io
  never allows re-publishing a version).
- Metadata error → fix `Cargo.toml`, and since the tag is already public, cut a
  new patch version rather than moving the tag.
