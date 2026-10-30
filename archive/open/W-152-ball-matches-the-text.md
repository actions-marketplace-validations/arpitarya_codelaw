---
type: Handoff
name: W-152
description: "Arpit, 2026-10-30 (voice): W-97 and W-126 sat green with `agent` while their next step was his — the queue checks never compare a row's ball with what it says. Add that check."
item: W-152
filed: 2026-10-30
filed_by: "Arpit (Cowork, Opus 5.5)"
updated: 2026-10-30
updated_by: "Arpit (Claude Code, Opus 5.5)"
ball: agent
closed: 2026-10-30
---

# W-152 — A row's ball must match what it says

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | `scripts/check-work-system.py`: a ball other than 🔴 fails when the inbox names the item, when the first unfinished step of its *At a glance* is 🔴, or when its row says the decider is what it waits for now ("needs Arpit", "Arpit's allow", "waits on the decider", "(inbox)") | a wrong colour fails the check, not a person's eye | ✅ 2026-10-30 |
| 2 | The same check: an unfinished step carrying a choice not yet ruled ("not ruled", "unruled", "if a step needs one", "only if asked") makes the row 🔴; a 🔴 row must be in the inbox or name the item it is red through | choices reach the decider | ✅ 2026-10-30 |
| 3 | Cases for both in `--self-test` (five rows that fail, two that pass), run in CI's `lint` job and at every landing; SR-WORK-OPEN-QUEUE rule 21a | it stays fixed | ✅ 2026-10-30 |

**Arpit, 2026-10-30 (voice):** *"So it is blocked on me, so it should be red. That is the first thing. Right now it is green. Why is it green?"* · *"Same thing goes for W 126."* · *"There was nothing that could have been done if I wouldn't have made the call, right? So that should also have been red."* · then, in text: *"file it and check on all the work items. Are they actually blocked or not? And correct their balls."*

**Model:** Sonnet (Claude Code): one check with cases.

## Why it happened

- The ball is set by hand by whichever session last edits the row; the checks verify length, links and format, not that the ball agrees with the text.
- A session marks an item green when any small task in it is possible, though the step that moves it is the decider's; two sessions editing the same rows copy a wrong colour forward (Cowork's merge did, for W-97).

## Corrected 2026-10-30 (Cowork, every item read)

| item | was | now | why |
|---|---|---|---|
| W-97 | 🟢 `agent` | 🔴 `arpit` + inbox | both halves need a public repo created — Claude Code's check refused `gh repo create` |
| W-126 | 🟢 `agent` (the `perf` choice unasked) | 🟢 `agent` | ruled the same day; the matrix and non-blocking `perf` are agent work now |
| the other twelve | — | unchanged | each one's next step checked: W-95, W-102, W-151 🔴 Arpit; W-142, W-143 🟡 W-139; W-103, W-105, W-129, W-139, W-141, W-145, W-146 🟢 with agent work ready |

## Definition of done

| DoD | what | state |
|---|---|---|
| 1 | both rules in the check, with cases; today's queue passes | ✅ 2026-10-30 — `check-work-system.py` green on `main`; `--self-test` 26 breaks named |
| 2 | SR-WORK-OPEN-QUEUE names the rule | ✅ rule 21a |

## Built 2026-10-30 (Claude Code)

- **Where:** `scripts/check-work-system.py` (`ball_agrees`, `at_a_glance`), called from the row loop of `check_queue` — the script CI's `lint` job and lockmaster's landing checks already run, with a `--self-test`. Not the hook's `check-open-work-inbox.py`: an item file's edit never runs that hook, and the four signals read item files too.
- **Structure first** (the hazard below): the inbox's links and `↳ **blocks:**` ids, and the state cell of the first step that is not plainly ✅ (a cell that starts ✅ and carries no 🔴 ⏳ 🟡 🔄). Phrases second, and only two short lists.
- **Quoted words do not count** — text in backticks or "…" is about a phrase; the first run failed this item's own row, which quotes the rule.
- **Would it have caught them?** W-97's old row said *"needs Arpit's allow"* → fails. W-126's old step 5 said *"if a step needs one"* → fails. Both checked against the rows as they stood before 960eda91.
- **Today's queue passes:** W-95, W-97, W-102 are 🔴 and in the inbox; W-142's 🟡 first step is W-139; W-143's 🔴 is a later step; W-103's and W-141's "then … to Arpit" are later steps, not matched.

## Blockers

None.

## Record affected

[SR-WORK-OPEN-QUEUE](../../records/0051_WORK-open-queue.md) (the ball rules).

## Hazards

- A phrase list misses phrasings; prefer the structure (the first non-✅ step's state cell) over prose where the file has one.

## Reference

[W-97](../../work/open/W-97-github-action.md) · [W-126](../../work/open/W-126-fast-pipeline.md)
