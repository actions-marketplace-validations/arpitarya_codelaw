---
id: yaml/gha-pin-sha
title: "GitHub Actions are pinned to a commit SHA"
languages: [yaml]
files: ["**/.github/workflows/*.yml", "**/.github/workflows/*.yaml", "**/action.yml", "**/action.yaml"]
fix: suggest
fix_reason: "the commit is the one `codelaw deps refresh` recorded for the ref, the one networked command (L5); the fix trusts that record, and a ref the snapshot has no entry for is pinned by hand"
fix_exception: value
stability: preview
since: 0.1.0
---

A `uses:` in a GitHub Actions workflow or composite action names a full 40-character commit SHA, not a tag or branch. Local actions (`./…`) and `docker://` images are not checked.

**Why:** A tag or branch can be moved to different code at any time, so a compromised or careless upstream changes what your workflow runs; a commit SHA can't change.

```codelaw
native: yaml/gha-pin-sha
```

## How to fix

1. Run `codelaw deps refresh`: it records the commit of each Action ref the workflows name, and the fix pins to it. Otherwise look up the commit the tag points to, for example with `git ls-remote https://github.com/<owner>/<repo> <tag>`.
2. Replace the tag with the 40-character SHA and keep the tag as a comment: `uses: actions/checkout@<sha> # v4.2.2`.
3. Let Dependabot or Renovate update the pinned SHAs.

## Examples

### YAML

#### Don't

```yaml
# .codelaw/deps-snapshot.json
{ "fetched": "2026-11-07", "actions": { "actions/checkout@v4": "11bd71901bbe5b1630ceea73d27597364c9af683" } }
# .github/workflows/ci.yml
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
```

#### Do

```yaml
# .codelaw/deps-snapshot.json
{ "fetched": "2026-11-07", "actions": { "actions/checkout@v4": "11bd71901bbe5b1630ceea73d27597364c9af683" } }
# .github/workflows/ci.yml
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4
```
