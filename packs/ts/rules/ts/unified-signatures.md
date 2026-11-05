---
id: ts/unified-signatures
title: "Overloads that one signature can say are one signature"
languages: [typescript, tsx]
fix: suggest
fix_reason: "merging two overloads changes the declared API callers see; the union is offered for an agent or a person to accept"
stability: preview
ported_from: [typescript-eslint/unified-signatures]
since: 0.5.0
---

Two adjacent overloads don't differ in only one parameter's type (write a union), or only by a trailing parameter (make it optional).

**Why:** Fewer overloads are easier to call and to read.

```codelaw
native: ts/unified-signatures
```

## How to fix

1. Write one signature with a union type or an optional parameter.

## Examples

### TypeScript

#### Don't

```ts
function f(x: string): void;
function f(x: number): void;
function f(x: unknown): void {}
```

#### Do

```ts
function f(x: string | number): void;
function f(x: unknown): void {}
```

### TSX

#### Don't

```tsx
function f(x: string): void;
function f(x: number): void;
function f(x: unknown): void {}
```

#### Do

```tsx
function f(x: string | number): void;
function f(x: unknown): void {}
```
