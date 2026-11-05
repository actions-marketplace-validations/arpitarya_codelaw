---
type: Proposal
title: Measure the security packs against public AI-code security sets, and move their fixes to suggest
description: "The four security packs get a published detection rate on a pinned, permissively licensed set of LLM-generated vulnerable code, and every manual security fix an agent can apply moves to suggest."
status: proposed
timestamp: 2026-11-05T19:50:00+05:30
---

# Measure the security packs

Filed 2026-11-05 (Cowork, Opus 5.5) from the research brief — *"file proposals
for all of the items"*. Item 3 of 7.

## 1 · In one line

**AI-written code is insecure often enough to be codelaw's best argument; the
security packs need a measured detection rate and fixes an agent can apply.**

## 2 · Evidence

| source | finding |
|---|---|
| Veracode GenAI Code Security Report (2025) | about 45% of AI-generated code tasks introduced a security flaw |
| Security Weaknesses of Copilot-Generated Code in GitHub Projects (TOSEM, arXiv 2310.02059) | 29.5% of Python and 24.2% of JavaScript snippets had a weakness, 43 CWE types; **given the static-analysis warnings, Copilot Chat fixed up to 55.5%** |
| Static Analysis as a Feedback Loop (arXiv 2508.14419) | Bandit in the loop: security issues > 40% → 13% on PythonSecurityEval |
| CORE, Microsoft (FSE 2024) | an LLM proposer + ranker fixed CodeQL/Sonar findings; 59.2% of Python files passed both the tool and a human reviewer |

**Read together:** a warning plus an agent fixes about half; a warning with a
ready diff should do better. A measured rate is a claim a reader can check.

## 3 · Today

- `security-js`, `security-ts`, `security-html`, `security-python` — about 130 rule files ([SR-RULE-PACKS](../../records/0101_rule-packs.md)).
- No published detection rate; the corpus counts measure noise on real repos, not recall on known-vulnerable code.
- The fix ladder review ([fix-ladder](fix-ladder.md)) covered these packs among the 717; security rules left `manual` were not re-checked against the 55.5% evidence.

## 4 · The proposal

- **A pinned recall set** in `fixtures/security-recall/`: LLM-generated vulnerable snippets with their CWE labels, from a set whose licence passes L2, plus their fixed versions.
- **`scripts/security-recall.py`** — runs `codelaw check` per snippet: per CWE, found / missed / false alarm on the fixed version. Output to `docs/` as a table, regenerated in CI.
- **One published number per pack** — recall on the set, false alarms on the fixed half — in each pack's README; never a claim the table does not back.
- **A second fix-ladder pass, security only:** each `manual` security rule whose fix is a local rewrite (parameterise a query, escape an output, drop `eval`) gets a `suggest` fix or a recorded reason.

## 5 · Laws touched

- **L2** — the set's licence decides whether it can be vendored; otherwise it is fetched by a script outside the engine and never shipped.
- **L4** — manual counts only fall; this pass is the security slice of that.
- **L5** — none: the script runs the engine; no model scores anything.

## 6 · Open questions for the decider

1. **Which set?** Candidates: PythonSecurityEval (Python), the Copilot study's 733 snippets, CyberSecEval's insecure-code set — licence of each to be checked before choosing.
2. **Publish recall in the README, or only in `docs/`?** Recommended: `docs/` table, one line in each pack README.
3. **JS/TS set** — none of the above is JS-first; build a small one from public CVE fixes, or wait.

## 7 · Graduation trigger

**The decider picks the set (§6.1) after its licence is reported** → a W-item: fixture, script, table, the security fix-ladder pass.

## Sources (read 2026-11-05)

- Veracode — AI-generated code poses major security risks in nearly half of all development tasks — <https://www.veracode.com/press-release/ai-generated-code-poses-major-security-risks-in-nearly-half-of-all-development-tasks-veracode-research-reveals/>
- Security Weaknesses of Copilot-Generated Code in GitHub Projects — <https://arxiv.org/abs/2310.02059>
- Static Analysis as a Feedback Loop — <https://arxiv.org/abs/2508.14419>
- CORE: Resolving Code Quality Issues using LLMs — <https://arxiv.org/abs/2309.12938>
