---
id: ts/no-misused-promises
title: "No async function where a `void` callback is expected"
languages: [typescript, tsx]
fix: manual
fix_reason: "whether the caller should wait for the work, or the work should handle its own errors, is the author's decision"
stability: preview
ported_from: [typescript-eslint/no-misused-promises]
tags: [types, async]
since: 0.4.4
---

A function that returns a Promise is not passed or assigned where a function returning `void` is
expected.

**Why:** Whoever calls a `void` callback ignores what it returns: the Promise is never awaited, a
rejection inside it goes unnoticed, and the caller carries on before the work is done
(`forEach`, an event listener, a `setTimeout`).

```codelaw
match:
  any:
    - kind: arrow_function
    - kind: function_expression
    - all:
        - any: [{ kind: identifier }, { kind: member_expression }]
        - inside: { kind: arguments, stopBy: neighbor }
type:
  $$MATCH: promise-to-void
message: "This function returns a Promise where a function returning `void` is expected: nobody awaits it"
```

## How to fix

1. Use a caller that waits: `for … of` with `await` instead of `forEach`, `await Promise.all(…map(…))`
   instead of a `forEach` of async work.
2. Or keep the callback synchronous and handle the Promise inside it:
   `() => { void work().catch(report); }`.

## Examples

### TypeScript

#### Don't

```ts
async function save(n: number): Promise<void> {}
[1, 2].forEach(async (n) => {
  await save(n);
});
```

#### Do

```ts
async function save(n: number): Promise<void> {}
export async function main(): Promise<void> {
  for (const n of [1, 2]) await save(n);
}
```

### TSX

#### Don't

```tsx
async function save(n: number): Promise<void> {}
[1, 2].forEach(save);
```

#### Do

```tsx
async function save(n: number): Promise<void> {}
export async function main(): Promise<void> {
  await Promise.all([1, 2].map(save));
}
```

## Differences from the source

Only typescript-eslint's `checksVoidReturn` is checked, for a function written in place (passed,
assigned or returned) or a function passed by name. `checksConditionals` is TypeScript's own
TS2801, reported once by `ts/type-error` (SR-TYPES decision 10), and `checksSpreads` is not checked.
A callback whose expected type has several signatures is reported only when every one returns `void`.
