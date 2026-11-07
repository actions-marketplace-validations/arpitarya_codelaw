---
id: ts/prefer-for-of
title: "A loop that only reads arr[i] is a for-of"
languages: [typescript, tsx]
fix: suggest
fix_reason: "`for … of` reads the array through its iterator, which a subclass or a sparse array can tell apart from indexing; the element is named the first of `item`, `item2`, … the file does not use, for the author to improve"
stability: preview
ported_from: [typescript-eslint/prefer-for-of]
since: 0.5.0
---

A counting `for` loop over `arr.length` whose index is used only to read `arr[i]` is written `for (const item of arr)`.

**Why:** The index is noise when only the element is wanted; `for…of` cannot go out of bounds or skip an element.

```codelaw
match:
  kind: for_statement
  all:
    - has: { field: initializer, kind: lexical_declaration, has: { kind: variable_declarator, all: [{ has: { field: name, pattern: $I } }, { has: { field: value, regex: '^0$' } }] } }
    - has: { field: condition, pattern: '$I < $A.length' }
    - has: { field: increment, any: [{ pattern: '$I++' }, { pattern: '++$I' }, { pattern: '$I += 1' }] }
    - has: { field: body, has: { stopBy: end, pattern: '$A[$I]' } }
    - not:
        has:
          field: body
          has:
            stopBy: end
            kind: identifier
            pattern: $I
            not: { inside: { kind: subscript_expression, pattern: '$A[$I]', not: { any: [{ inside: { kind: assignment_expression, field: left } }, { inside: { kind: augmented_assignment_expression, field: left } }, { inside: { kind: update_expression } }] } } }
message: "`$I` only indexes `$A`; loop with `for (const … of $A)`"
fix:
  - edit-text: { pattern: '\Afor\s*\([^;]*;[^;]*;[^)]*\)', replace: "for (const $FRESH(item) of $A)" }
  - edit-text: { pattern: '\b$A\[\s*$I\s*\]', replace: "$FRESH(item)" }
```

## How to fix

1. Write `for (const item of $A)`, naming the element.
2. Replace each `$A[$I]` in the body with that name.

## Examples

### TypeScript

#### Don't

```ts
for (let i = 0; i < names.length; i++) {
  greet(names[i]);
}
```

#### Do

```ts
for (const item of names) {
  greet(item);
}
```

### TSX

#### Don't

```tsx
for (let i = 0; i < names.length; i++) {
  greet(names[i]);
}
```

#### Do

```tsx
for (const item of names) {
  greet(item);
}
```
