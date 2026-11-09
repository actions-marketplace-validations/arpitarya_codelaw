---
type: Handoff
name: W-174
description: "Python type, flow and class-hierarchy inference for the W-139 python rows a single file's syntax cannot answer: 38 Pylint and Ruff ports, each named with the question it needs answered."
item: W-174
filed: 2026-11-08
filed_by: "Claude Code (session 0a172e1f), W-139 strand W-139-py"
updated: 2026-11-08
updated_by: "Claude Code (Sonnet 5.5), W-174 agent"
ball: arpit
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
| 1 | Propose the Python counterpart of SR-TYPES: what answers each question above, and its cost against L8 | ✅ [SR-PY-INFERENCE](../../records/0120_python-inference.md) (0120): four lazy layers — scope facts on the per-file model, a flow pass (`codelaw_semantic::python_flow`), an in-file class table, literal typing; no type checker; the one question for the decider is its decision 8 |
| 2 | Build the cheapest layer that answers a group (scope facts first) | ✅ scope facts (`py_scope.rs`), flow (`semantic/py_flow.rs` + `native/py_flow.rs`), class hierarchy in-file (`py_classes.rs`), value types in one file (`py_values.rs`) |
| 3 | Write each row fresh (Ruff 0.16.9 source and tests; Pylint 4.1.3 behaviour from its functional tests only, never code, L2), its cases and fix class; `codelaw test packs/python` 0 failed | ✅ 30 of 38 rows (below); the other 8 are in *Left* with their exact question |

## Built (2026-11-08) — 30 `preview` rules, in no preset

| layer | rules (upstream) | fix |
|---|---|---|
| scope facts (5) | `nonlocal-without-binding` (PLE0117), `builtin-attribute-shadowing` (A003), `missing-f-string-syntax` (RUF027), `used-dummy-variable` (RUF052), `unused-private-member` (Pylint) | RUF027 and RUF052 `suggest` (Ruff's own fixes are unsafe: they change behaviour, L4 decision 3), the rest `manual` (`value` ×2, `restructure`) |
| flow (6) | `used-before-assignment`, `possibly-used-before-assignment`, `undefined-loop-variable`, `possibly-unused-variable`, `looping-through-iterator`, `contextmanager-generator-missing-cleanup` | all `manual` (`value` ×4, `restructure` ×2) |
| class hierarchy, in one file (13) | `inconsistent-mro`, `bad-super-call`, `non-parent-init-called`, `super-init-not-called`, `subclassed-final-class`, `overridden-final-method`, `invalid-enum-extension`, `abstract-class-instantiated`, `abstract-method`, `method-hidden`, `invalid-overridden-method`, `arguments-differ`, `arguments-renamed` | `bad-super-call` `suggest` (names the enclosing class); the rest `manual` |
| value types from literals and annotations (6) | `dict-iter-missing-items` (PLE1141), `modified-iterating-set` (PLE4703), `bad-string-format-type` (PLE1307), `comparison-with-callable`, `positional-only-arguments-expected`, `blocking-http-call-httpx-in-async-function` (ASYNC212) | PLE1141 and PLE4703 `suggest` (`.items()`, `.copy()`), the rest `manual` |

Each rule page says what it does not read. Behaviour pinned from the upstream tests: the flow rules were checked against Pylint's `used_before_assignment`, `undefined_loop_variable`, `looping_iterator`, `unused_variable` and `contextmanager_generator_missing_cleanup` functional cases, and run over 150 files of the CPython 3.14 standard library (hits read: the `try`-body-only reads in `except` handlers, `if` without `else`, loop variables read after the loop — as Pylint reports them; three false positives found and fixed on the way: chained assignments `a = b = 0`, a builtin read at module level before the module binds it, and `a and (b := f())` binding `b` where it held). **Corpus (2026-11-09):** the 30 rules' counts are in `packs/python/corpus.toml` (`corpus.py run --pack python --update`, then once more without it: 0 failed, a fixpoint; no existing count moved). 22 are 0; the rest were sampled: `used-dummy-variable` 132 (comprehension and local-alias names in `rich`, as Ruff reports them), `missing-f-string-syntax` 5 (Starlette path templates such as `"/{param:float}"` with `param` in scope, as Ruff reports them), `undefined-loop-variable` 4, `possibly-used-before-assignment` 2, `possibly-unused-variable` 2 (`locals()`), `super-init-not-called` 3 (Starlette's `BackgroundTasks` and two `Response` subclasses, whose `__init__` does not call the base's) — all true by Pylint's definitions. One was wrong: `arguments-differ` read Flask's `dispatch_request(self, **kwargs: t.Any)` as a positional parameter, because tree-sitter wraps an annotated `*args`/`**kwargs` in a `typed_parameter`; fixed in `py_classes.rs` (and the same read in `py_values.rs`'s `positional-only-arguments-expected`), pinned by a case each, count 0.

## Left — 8 rows, each with the question it needs

| row | question | layer |
|---|---|---|
| `signature-differs` (W0222) | *do this override's and its base's signatures differ in the way Pylint calls "signature"* (defaults, keyword-only, variadics) | the class table, once `arguments-differ`'s edge cases are pinned against the functional tests; and the base's methods from another file |
| `access-member-before-definition` (E0203) | *is this `self.x` read before any method assigns it* — the order of attribute definitions across a class's methods and its bases | an attribute table in `py_classes.rs` (in-file, buildable next) |
| `not-async-context-manager` (E1701), `non-iterator-returned` (E0301) | *what does this class's `__aenter__`/`__aexit__`, `__iter__`'s return value evaluate to* — a method's returned value | in-file for a class defined here; a type for anything else |
| `redefined-variable-type` (R0204, an opt-in extension) | *is the type of a name's value the same at each assignment in a scope* — branch-aware literal typing | literal typing + flow join (buildable next) |
| `invalid-format-index`, `missing-format-attribute` | *what is the argument a `str.format` field indexes or reads an attribute of* — a literal's type and its attribute table | literal typing (buildable next, small tables) |
| `boolean-datetime` (W1502) | *is this a `datetime.time`* — and Pylint reports it only below Python 3.5, under codelaw's 3.9+ floor it can never fire | **a decision**: re-rule the row `not-applicable` (recommended) |

Across files: every override, abstract and `__init__` row stops at a base class this file imports; the project graph carrying class facts (bases, methods, decorators) would let `py_classes.rs` follow them. That is the next record step, not built.

## Blockers

None for the in-file rows. **For the decider (not blocking):** [SR-PY-INFERENCE](../../records/0120_python-inference.md) decision 8 — should the rows that need a call's return or an imported class's shape be answered by *the project's own Python type checker, found and asked like the project's TypeScript* (`ty`, pyright or mypy; codelaw installs nothing, L11 decision 2)? And whether `boolean-datetime` is re-ruled `not-applicable`.

## Reference

[W-139](W-139-rule-parity-build.md) (where the rows came from: its *Needs native — W-139-py* table and the strand's *not written* lists) · [W-105](W-105-type-aware-engine.md) (the TypeScript type program) · [SR-TYPES](../../records/0117_types.md)
