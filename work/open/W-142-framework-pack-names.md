---
type: Handoff
name: W-142
description: "Arpit, 2026-10-30 (voice): the registry names of the packs W-139 builds — five PyPI framework packs (pending publishers) and the npm testing and nextjs packs (was W-147, first publish by hand) — bound just before the release that carries them. Merged 2026-10-30."
item: W-142
filed: 2026-10-30
filed_by: "Arpit (Cowork, Opus 5.5)"
updated: 2026-11-05
updated_by: "Claude Code (Opus 5.5), W-145"
ball: arpit
---

# W-142 — Registry names for the new packs

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | Wait until W-139 has the five packs ready to release | names bound when they are needed | 🟡 waits on W-139 |
| 2 | Claude, in Chrome: pypi.org → *Publishing → Add a pending publisher*, once per name; Arpit checks and submits | `publish.yml` can create each project | 🔴 Arpit submits |
| W-147 | npm: first publish of the `testing` and `nextjs` packs' names — merged in; its steps under [§ W-147](#merged-in-w-147) | one item, not two | ⏳ — see § |

**Merged 2026-10-30 (Arpit, text):** *"There are too many work items. See if we can review all the work items and merge them into few of them."* → *"go ahead, merge them."* Merged in: W-147.

**Arpit, 2026-10-30 (voice):** *"Give each framework its own pack."* · on the names: *"with anything that you recommend."*

**Model:** Cowork with Claude in Chrome; Arpit submits.

**Arpit, 2026-11-05 (voice docket):** *"Go with the recommendation."* — nothing now; the pending publishers, the four npm first publishes and their trusted publishers are done just before the release that carries the packs, once W-139 is ready.

## The names (all free on PyPI, checked 2026-10-30 08:30)

| pack | PyPI | its Fux index (L6 decision 12) |
|---|---|---|
| Django | `codelaw-rules-django` | `codelaw-rules-django-fux-index` |
| FastAPI | `codelaw-rules-fastapi` | `codelaw-rules-fastapi-fux-index` |
| Airflow | `codelaw-rules-airflow` | `codelaw-rules-airflow-fux-index` |
| NumPy | `codelaw-rules-numpy` | `codelaw-rules-numpy-fux-index` |
| pandas | `codelaw-rules-pandas` | `codelaw-rules-pandas-fux-index` |

Form values for each: owner `arpitarya` · repository `codelaw` · workflow `publish.yml` · environment `pypi`.

**Added 2026-11-05 by W-145 (closed):** the `tailwind` pack and its provider need their names too, before the release that carries them —

| package | registry | its Fux index |
|---|---|---|
| `codelaw-rules-tailwind` | PyPI (pending publisher, as above; add it to `publish.yml`'s `PENDING`) | `codelaw-rules-tailwind-fux-index` |
| `@codelaw/rules-tailwind` | npm (first publish by hand, as `nextjs`'s in § W-147) | `@codelaw/rules-tailwind-fux-index` |
| `@codelaw/provider-tailwind` | npm (first publish by hand; `publish.yml` uploads it in the binaries' tier, before the packs) | — (holds no rules) |

**Added 2026-11-05 by W-166:** the browser-support data package (L6 decision 15) needs its names before the release that first carries it —

| package | registry | its Fux index |
|---|---|---|
| `codelaw-data-browsers` | PyPI (pending publisher, as above; add it to `publish.yml`'s `PENDING`) | — (holds no rules) |
| `@codelaw/data-browsers` | npm (first publish by hand) | — (holds no rules) |


**PyPI only:** L6 ships `python` and `security-python` on PyPI alone (*"each registry carries only the packs its users install"*); these five follow them. Said by voice as also on npm (`@codelaw/rules-django` …) — not filed; say so if npm is wanted, and L6's table changes with it.

## Definition of done

| DoD | what | state |
|---|---|---|
| 1 | ten pending publishers saved | ⏳ |
| 2 | the release that ships the packs creates the five projects | ⏳ |

## Blockers

🟡 W-139.

## Record affected

[SR-PACKAGES](../../records/0107_packages.md) · L6 decision 11 (W-139 writes the amendment) · RELEASING.md.

## Hazards

- A pending publisher reserves nothing; bind close to the release.
- [W-150](W-151-release-0-4-1.md) (was B-063): Fux-index packages are not uploaded by `publish.yml` yet.

## Reference

[W-139](W-139-rule-parity-build.md) · [W-134](W-151-release-0-4-1.md) (the same steps for `deps`)

## Merged in: W-147

<a id="merged-in-w-147"></a>

**W-147 — npm: first publish of the `testing` and `nextjs` packs' names**, merged into W-142 on 2026-10-30; its file is archived at [`archive/open/W-147-testing-nextjs-npm-first-publish.md`](../../archive/open/W-147-testing-nextjs-npm-first-publish.md). As it stood:

### At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | Wait until W-139 has the two packs ready to release | the names are bound when they are needed | 🟡 waits on W-139 |
| 2 | Arpit publishes the four names below once by hand, as RELEASING.md does for a new pack | the names exist on npm | 🔴 Arpit |
| 3 | Add `arpitarya/codelaw` · `publish.yml` · `npm` as each package's trusted publisher | every later release publishes them from CI | 🔴 Arpit |

**Arpit, 2026-10-30 (voice):** testing pack — *"Let's create it."* · *"The next JS pack. Let's create it."*

**Model:** NONE for steps 2–3 — npm has no pending publisher; only Arpit's npm login can publish a new name.

### The names

| pack | npm | its Fux index (L6 decision 12) |
|---|---|---|
| testing (Jest, Vitest, Testing Library, Playwright) | `@codelaw/rules-testing` | `@codelaw/rules-testing-fux-index` |
| Next.js | `@codelaw/rules-nextjs` | `@codelaw/rules-nextjs-fux-index` |

**npm only:** Python's test rules live in `python`; Next.js is JavaScript. Under the `@codelaw` scope, so no outside name can take them.

### Definition of done

| DoD | what | state |
|---|---|---|
| 1 | All four names on npm with a trusted publisher | ⏳ |
| 2 | The next release publishes both packs from `publish.yml` | ⏳ |

### Blockers

🟡 W-139.

### Record affected

[SR-PACKAGES](../../records/0107_packages.md) (the names) · L6 decision 11 (W-139 writes the amendment) · RELEASING.md.

### Hazards

- [W-150](W-151-release-0-4-1.md) (was B-063): `publish.yml` does not upload Fux-index packages yet; the fux-index names are bound by hand regardless.
- Until both names have a trusted publisher, `publish.yml`'s trust check stops a release carrying the packs.

### Reference

[W-139](W-139-rule-parity-build.md) · [W-144](../../archive/open/W-144-rule-parity-round-2.md) · [W-135](W-151-release-0-4-1.md) (the same steps for `deps`) · [W-142](W-142-framework-pack-names.md) (PyPI's five)
