# Contributor Review Guide

This guide is for anyone reviewing a pull request or closing an issue in this repository.
It complements [CONTRIBUTING.md](../CONTRIBUTING.md), which is written for the author of a
change. Where the two overlap, CONTRIBUTING.md defines the standard and this guide defines
how a reviewer confirms it was met.

The governing rule is the [Definition of Done for Issue
Closure](../CONTRIBUTING.md#definition-of-done-for-issue-closure). A review is not complete
until the reviewer has confirmed, on the merged branch, that the artefacts the PR claims to
have produced actually exist.

## Why this guide exists

An audit of Wave 8 found four issues closed with no corresponding artefact on `main`. In each
case a pull request merged under a matching title, and in each case the work was not present
afterwards. The failures were not caught because nothing in the process asked a reviewer to
look. See [Worked examples](#worked-examples-of-the-failure-mode) below.

## Reviewer checklist

Run through this for every PR. The artefact checks are not optional and are not satisfied by
reading the diff summary on GitHub.

### 1. Scope and linkage

- [ ] The PR body links the issue with `Closes #<number>`.
- [ ] The change matches the linked issue's acceptance criteria — no more, no less.
- [ ] Branch name uses `feat/`, `fix/`, or `docs/`; commits follow Conventional Commits.

### 2. Artefact verification (required)

- [ ] The PR body contains an **Artefacts** section naming every file or artefact it creates
      or changes, by path.
- [ ] Every path named in that section appears in the PR's **Files changed** tab.
- [ ] Each named artefact is non-empty and contains the claimed implementation, not a
      placeholder, a stub, or an empty file.
- [ ] After merge, each named artefact exists on `main`. Verify against the merged branch:

```bash
git fetch origin main
git ls-tree -r --name-only origin/main -- <path>   # prints the path if it exists
git show origin/main:<path> | head                 # prints the content, fails if absent
```

- [ ] For a claimed test suite, the tests are discovered and executed by the runner, not
      merely present as files. Check CI output for the new test names, or run the suite.
- [ ] For a claimed CI workflow, the workflow file is under `.github/workflows/` and has
      appeared in the Actions tab of the merge commit.

If any artefact named by the PR is missing after merge, reopen the issue and say so in the
issue thread with the output of the command that shows the absence.

### 3. Validation

- [ ] Lint passed for each affected area.
- [ ] Tests passed for each affected area, and new behaviour is covered by new tests.
- [ ] CI is green on the head commit, not only on an earlier push.
- [ ] Screenshots or video are attached for UI changes.

### 4. Documentation

- [ ] Docs updated when behaviour, setup, or usage changed, including the relevant
      `document/` guide.
- [ ] Documentation-only PRs still name their artefacts — a documentation issue is closed by
      a document existing at a stated path.

## Closing an issue

An issue may be closed in exactly one of two ways.

**With a merged pull request.** The PR names its artefacts, the reviewer has confirmed those
artefacts exist on the merged branch, and the issue is closed by the `Closes #<number>` link
or manually with a comment naming the merge commit.

**Without a merged pull request.** This requires an explicit written reason left as a comment
on the issue before it is closed. The reason must state which of the following applies and
why:

- **Superseded** — name the issue or PR that replaced it.
- **Already satisfied** — name the existing artefact path and the commit that added it.
- **Not doing** — state the decision and who made it.
- **Cannot reproduce / invalid** — state what was tried.
- **Duplicate** — link the original issue.

"Done", "fixed", "completed", or a bare close with no comment is not an acceptable reason.
An issue closed without either a linked merged PR or a written reason should be reopened by
anyone who notices it.

## Worked examples of the failure mode

These are the four Wave 8 cases from the audit. All four were closed against a merged pull
request with a matching title. None of the artefacts were on `main` afterwards. Each row
gives the command that demonstrates the absence, so the same check can be run before closing
a comparable issue.

| # | Claimed artefact | Expected path | Verification that would have caught it |
|---|---|---|---|
| 1 | Webapp CI workflow | `.github/workflows/webapp.yml` | `ls .github/workflows` — only `backend.yml`, `data-processing.yml`, and `onchain.yml` are present; no webapp workflow has ever run in the Actions tab. |
| 2 | Playwright end-to-end suite | `apps/webapp/playwright.config.ts`, `apps/webapp/e2e/` | `find . -iname '*playwright*' -not -path '*/node_modules/*'` returns nothing; `playwright` is absent from `apps/webapp/package.json`. |
| 3 | Webapp i18n catalogs | `apps/webapp/messages/*.json` (or `locales/`) | `grep -E 'intl\|i18n' apps/webapp/package.json` returns nothing and no catalog directory exists; no component imports a translation hook. |
| 4 | `notification_interface` test module | `apps/onchain/contracts/notification_interface/src/test.rs` | The crate contains only `Cargo.toml` and `src/lib.rs`, and `lib.rs` has no `#[cfg(test)] mod test`. Sibling contracts such as `crowdfund_vault` and `vesting-wallet` do carry `src/test.rs`, which is the convention this one was claimed to follow. |

The shared pattern is that each was accepted on the strength of the PR title and description
alone. In every case a single command against the merged branch would have shown the work was
not there. That command is now a required step in this checklist.

## When a review blocks

A PR may be blocked when:

- The Artefacts section is missing or names paths absent from the diff.
- A claimed file exists but is empty or a stub.
- Scope does not match the linked issue.
- Tests or lint were skipped without a stated reason.
- Required docs are missing.
