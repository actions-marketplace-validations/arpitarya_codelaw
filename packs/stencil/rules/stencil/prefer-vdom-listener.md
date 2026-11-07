---
id: stencil/prefer-vdom-listener
title: "DOM events are handled in render(), not with @Listen"
languages: [typescript, tsx]
fix: suggest
fix_reason: "in a component with no `render()` or `hostData()`, the @Listen on a host event becomes an `on<Event>` handler on the `<Host>` a new `render()` draws; the component then renders a host it didn't before, which the reviewer confirms. Any other @Listen is manual"
fix_exception: restructure
stability: preview
ported_from: [stencil/prefer-vdom-listener]
since: 0.1.0
---

A component handles DOM events on itself (`click`, `keydown`, …) with an `on…` handler in
`render()`, not `@Listen`; `@Listen` with a `target` is fine.

**Why:** Handlers in `render()` are scoped to the element they're on and cheaper than a host-wide
listener.

```codelaw
native: stencil/prefer-vdom-listener
```

## How to fix

1. Attach the handler in `render()` (e.g. `onClick={…}` on `<Host>`).
2. Remove the `@Listen`.

## Examples

### TypeScript

#### Don't

```ts
@Component({ tag: 'my-card' })
export class Card {
  @Listen('click') onClick() {}
}
```

#### Fixed

```ts
@Component({ tag: 'my-card' })
export class Card {
  onClick() {}
  render() {
    return h(Host, { onClick: (ev) => this.onClick(ev) });
  }
}
```

#### Do

```ts
@Component({ tag: 'my-card' })
export class Card {
  @Listen('resize', { target: 'window' }) onResize() {}
}
```

### TSX

#### Don't

```tsx
@Component({ tag: 'my-card' })
export class Card {
  @Listen('click') onClick() {}
}
```

#### Fixed

```tsx
@Component({ tag: 'my-card' })
export class Card {
  onClick() {}
  render() {
    return h(Host, { onClick: (ev) => this.onClick(ev) });
  }
}
```

#### Do

```tsx
@Component({ tag: 'my-card' })
export class Card {
  @Listen('resize', { target: 'window' }) onResize() {}
}
```
