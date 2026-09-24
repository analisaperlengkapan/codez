## Summary

What does this change do, and why?

## Related issues

Closes #

## Changes

- 

## Verification

List the commands you ran and their result:

- [ ] `cargo fmt --all -- --check`
- [ ] `cargo clippy --workspace --all-targets -- -D warnings`
- [ ] `cargo test --workspace`
- [ ] `(cd crates/frontend && trunk build)`
- [ ] `npx playwright test --project=chromium` (if UI behaviour changed)
- [ ] `python scripts/audit_responsive.py` (if layout/CSS changed)

## Screenshots

For UI changes, include before/after screenshots. Windows UI changes should be
audited at 390 px / 768 px / 1440 px.

## Notes for reviewers

Anything worth calling out — trade-offs, follow-ups, or areas to focus on.
