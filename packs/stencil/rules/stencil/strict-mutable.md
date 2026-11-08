---
id: stencil/strict-mutable
title: "mutable: true only on props the component changes"
languages: [typescript, tsx]
fix: auto
fix_reason: "`mutable: true` only allows the component to assign the prop, and nothing in it does, so removing it (and an options object it leaves empty) changes nothing"
stability: preview
ported_from: [stencil/strict-mutable]
since: 0.1.0
---

A `@Prop({ mutable: true })` is assigned somewhere in the component.

**Why:** `mutable: true` says the component changes the prop; if it never does, the flag only hides
that the prop should be `readonly`.

```codelaw
native: stencil/strict-mutable
```

## How to fix

1. Remove `mutable: true` and make the prop `readonly`, or assign it where the component means to.

## Examples

### TypeScript

#### Don't

```ts
@Component({ tag: 'my-card' })
export class Card {
  /** The count. */
  @Prop({ mutable: true }) count = 0;
}
```

#### Fixed

```ts
@Component({ tag: 'my-card' })
export class Card {
  /** The count. */
  @Prop() count = 0;
}
```

#### Do

```ts
@Component({ tag: 'my-card' })
export class Card {
  /** The count. */
  @Prop({ mutable: true }) count = 0;
  private inc() {
    this.count++;
  }
}
```

### TSX

#### Don't

```tsx
@Component({ tag: 'my-card' })
export class Card {
  /** The count. */
  @Prop({ mutable: true }) count = 0;
}
```

#### Fixed

```tsx
@Component({ tag: 'my-card' })
export class Card {
  /** The count. */
  @Prop() count = 0;
}
```

#### Do

```tsx
@Component({ tag: 'my-card' })
export class Card {
  /** The count. */
  @Prop({ mutable: true }) count = 0;
  private inc() {
    this.count++;
  }
}
```
