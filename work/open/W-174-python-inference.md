---
type: Handoff
name: W-174
description: "Python type, flow and class-hierarchy inference for the W-139 python rows a single file's syntax cannot answer: 38 Pylint and Ruff ports, each named with the question it needs answered."
item: W-174
filed: 2026-11-08
filed_by: "Claude Code (session 0a172e1f), W-139 strand W-139-py"
updated: 2026-11-08
updated_by: "Claude Code (session 0a172e1f), W-139 strand W-139-py"
ball: agent
---

# W-174 — Python inference for the W-139 rows that need it

## Why this exists

codelaw has a type program for TypeScript only ([SR-TYPES](../../records/0117_types.md), W-105: `type:` questions are a load error outside `typescript`/`tsx`). For Python it has the per-file semantic model (`crates/codelaw-semantic`: scopes, bindings, references, imports), which answers *which binding does this name resolve to* — not *what type is this value*, *which class is this a subclass of*, *what does this call return* or *is this name bound on every path here*. W-139's python strand left these rows unbuilt rather than fake them syntactically (W-139's hazard: *a rule that needs types waits … do not fake it syntactically*). This item is where they wait, so none is orphaned when W-139 closes.

## The rows, by the question they need

| question | rows (upstream id) |
|---|---|
| **flow: is the name bound on this path** (definite assignment over branches, loops, `try`) | Pylint `used-before-assignment`, `possibly-used-before-assignment`, `undefined-loop-variable`, `possibly-unused-variable`, `contextmanager-generator-missing-cleanup`, `looping-through-iterator` |
| **class hierarchy across files** (the MRO, a base's methods and signatures) | Pylint `arguments-differ`, `arguments-renamed`, `signature-differs`, `invalid-overridden-method`, `method-hidden`, `inconsistent-mro`, `abstract-method`, `abstract-class-instantiated`, `access-member-before-definition`, `non-parent-init-called`, `super-init-not-called`, `bad-super-call`, `overridden-final-method`, `subclassed-final-class`, `invalid-enum-extension`, `not-async-context-manager`, `non-iterator-returned` |
| **the type of a value** (a call's return, a parameter's declared type followed through bindings) | Pylint `boolean-datetime`, `comparison-with-callable`, `positional-only-arguments-expected`, `redefined-variable-type`, `invalid-format-index`, `missing-format-attribute`; Ruff `ASYNC212` (an `httpx.Client`'s method: the receiver's type), `PLE1141` (`dict-iter-missing-items`: the iterable is a dict), `PLE1307` (`bad-string-format-type`: each argument's type against its `%` conversion), `PLE4703` (`modified-iterating-set`: the iterable is a set) |
| **scope facts the semantic model does not expose yet** | Ruff `PLE0117` (`nonlocal-without-binding`: no enclosing function binds the name), `A003` (a class-scope binding read from the class body), `RUF027` (`missing-f-string-syntax`: which `{name}` in a plain string names a variable in scope), `RUF052` (`used-dummy-variable`: a `_`-prefixed local that is read), Pylint `unused-private-member` (class-scope reference tracking) |

38 rows. Each is `class = port` in `work/proposals/rule-parity/ruff.csv` or `pylint.csv`, so W-139's DoD 1 counts it until it is built or this item re-rules it.

## Steps

| step | what we do | state |
|---|---|---|
| 1 | Propose the Python counterpart of SR-TYPES: what answers each question above (the semantic model extended, a flow pass in `codelaw-semantic`, or an existing type checker run like TypeScript's — the last needs L2 and L11 rulings), and its cost against L8 | ⏳ |
| 2 | Build the cheapest layer that answers a group (scope facts first: they need no new dependency) | ⏳ |
| 3 | Write each row fresh (Ruff 0.16.9 source and tests; Pylint 4.1.3 behaviour only, never code, L2), its cases and fix class; `codelaw test packs/python` 0 failed | ⏳ |

## Blockers

None to start step 1. A type checker as a dependency would need the decider (L11).

## Reference

[W-139](W-139-rule-parity-build.md) (where the rows came from: its *Needs native — W-139-py* table and the strand's *not written* lists) · [W-105](W-105-type-aware-engine.md) (the TypeScript type program) · [SR-TYPES](../../records/0117_types.md)
