---
id: stencil/methods-must-be-public
title: "@Method() members are public"
languages: [typescript, tsx]
fix: auto
fix_reason: "TypeScript's `private` and `protected` exist only for the compiler, so removing one from a @Method() changes nothing at run time"
stability: preview
ported_from: [stencil/methods-must-be-public]
since: 0.1.0
---

A `@Method()` isn't marked `private` or `protected`.

**Why:** A `@Method()` is part of the element's public API; hiding it in TypeScript contradicts
that.

```codelaw
native: stencil/methods-must-be-public
```

## How to fix

1. Remove `private`/`protected`, or remove `@Method()` if it isn't public API.

## Examples

### TypeScript

#### Don't

```ts
@Component({ tag: 'my-card' })
export class Card {
  /** Opens the card. */
  @Method() private async open() {}
}
```

#### Do

```ts
@Component({ tag: 'my-card' })
export class Card {
  /** Opens the card. */
  @Method() async open() {}
}
```

### TSX

#### Don't

```tsx
@Component({ tag: 'my-card' })
export class Card {
  /** Opens the card. */
  @Method() private async open() {}
}
```

#### Do

```tsx
@Component({ tag: 'my-card' })
export class Card {
  /** Opens the card. */
  @Method() async open() {}
}
```
