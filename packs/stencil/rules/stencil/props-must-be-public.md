---
id: stencil/props-must-be-public
title: "@Prop() members are public"
languages: [typescript, tsx]
fix: auto
fix_reason: "TypeScript's `private` and `protected` exist only for the compiler, so removing one from a @Prop() changes nothing at run time"
stability: preview
ported_from: [stencil/props-must-be-public]
since: 0.1.0
---

A `@Prop()` isn't marked `private` or `protected`.

**Why:** A prop is set from outside the component; hiding it in TypeScript contradicts that.

```codelaw
native: stencil/props-must-be-public
```

## How to fix

1. Remove `private`/`protected`, or make it a `@State()` if it is internal.

## Examples

### TypeScript

#### Don't

```ts
@Component({ tag: 'my-card' })
export class Card {
  /** The heading. */
  @Prop() private readonly heading: string;
}
```

#### Do

```ts
@Component({ tag: 'my-card' })
export class Card {
  /** The heading. */
  @Prop() readonly heading: string;
}
```

### TSX

#### Don't

```tsx
@Component({ tag: 'my-card' })
export class Card {
  /** The heading. */
  @Prop() private readonly heading: string;
}
```

#### Do

```tsx
@Component({ tag: 'my-card' })
export class Card {
  /** The heading. */
  @Prop() readonly heading: string;
}
```
