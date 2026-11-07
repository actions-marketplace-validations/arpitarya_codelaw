---
id: stencil/no-unused-watch
title: "@Watch watches a @Prop or @State"
languages: [typescript, tsx]
fix: suggest
fix_reason: "the choices are the @Prop and @State names within two edits of the watched one, nearest first and the default, or removing the @Watch; the handler then runs, or never could, which the reviewer confirms"
fix_exception: value
stability: preview
ported_from: [stencil/no-unused-watch]
since: 0.1.0
---

`@Watch('name')` names a `@Prop` or `@State` of the same component.

**Why:** Watching anything else never fires, so the handler is dead code.

```codelaw
native: stencil/no-unused-watch
```

## How to fix

1. Correct the name to the @Prop or @State it should watch, or remove the watcher.

## Examples

### TypeScript

#### Don't

```ts
@Component({ tag: 'my-card' })
export class Card {
  @State() open = false;
  @Watch('opened') onOpen() {}
}
```

#### Fixed (open)

```ts
@Component({ tag: 'my-card' })
export class Card {
  @State() open = false;
  @Watch('open') onOpen() {}
}
```

#### Fixed (remove)

```ts
@Component({ tag: 'my-card' })
export class Card {
  @State() open = false;
  onOpen() {}
}
```

#### Do

```ts
@Component({ tag: 'my-card' })
export class Card {
  @State() open = false;
  @Watch('open') onOpen() {}
}
```

### TSX

#### Don't

```tsx
@Component({ tag: 'my-card' })
export class Card {
  @State() open = false;
  @Watch('opened') onOpen() {}
}
```

#### Fixed (open)

```tsx
@Component({ tag: 'my-card' })
export class Card {
  @State() open = false;
  @Watch('open') onOpen() {}
}
```

#### Fixed (remove)

```tsx
@Component({ tag: 'my-card' })
export class Card {
  @State() open = false;
  onOpen() {}
}
```

#### Do

```tsx
@Component({ tag: 'my-card' })
export class Card {
  @State() open = false;
  @Watch('open') onOpen() {}
}
```
