---
id: js/prefer-observer-apis
title: "ResizeObserver or IntersectionObserver rather than reading layout in resize or scroll listeners"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "an observer has a different callback shape and lifetime; the rewrite is a design change"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-observer-apis]
since: 0.5.0
---

A `resize` or `scroll` listener that reads layout (`offsetWidth`, `offsetHeight`, `clientWidth`, `clientHeight`, `scrollHeight`, `getBoundingClientRect()`) forces a synchronous layout on every event. `ResizeObserver` reports size changes and `IntersectionObserver` visibility, without the forced layout. A listener that only reads the scroll position is left.

**Why:** Layout reads in high-frequency events are a common source of jank.

```codelaw
match:
  kind: call_expression
  all:
    - has: { field: function, kind: member_expression, has: { field: property, regex: '^addEventListener$' } }
    - has:
        field: arguments
        all:
          - has: { nthChild: 1, kind: string, regex: '^["''](resize|scroll)["'']$' }
          - has:
              nthChild: 2
              any: [{ kind: arrow_function }, { kind: function_expression }]
              has: { stopBy: end, kind: property_identifier, regex: '^(offsetWidth|offsetHeight|clientWidth|clientHeight|scrollHeight|scrollWidth|getBoundingClientRect)$' }
message: "Observe the element instead of reading layout on every event"
```

## How to fix

1. For size, use `new ResizeObserver((entries) => …).observe(element)` and read `entry.contentRect`.
2. For visibility, use `new IntersectionObserver((entries) => …)` and read `entry.isIntersecting`.

## Examples

### JavaScript

#### Don't

```js
window.addEventListener("resize", () => {
  panel.classList.toggle("small", panel.offsetWidth < 500);
});
```

#### Do

```js
new ResizeObserver(([entry]) => {
  panel.classList.toggle("small", entry.contentRect.width < 500);
}).observe(panel);
```


### JSX

#### Don't

```jsx
window.addEventListener("resize", () => {
  panel.classList.toggle("small", panel.offsetWidth < 500);
});
```

#### Do

```jsx
new ResizeObserver(([entry]) => {
  panel.classList.toggle("small", entry.contentRect.width < 500);
}).observe(panel);
```


### TypeScript

#### Don't

```ts
window.addEventListener("resize", () => {
  panel.classList.toggle("small", panel.offsetWidth < 500);
});
```

#### Do

```ts
new ResizeObserver(([entry]) => {
  panel.classList.toggle("small", entry.contentRect.width < 500);
}).observe(panel);
```


### TSX

#### Don't

```tsx
window.addEventListener("resize", () => {
  panel.classList.toggle("small", panel.offsetWidth < 500);
});
```

#### Do

```tsx
new ResizeObserver(([entry]) => {
  panel.classList.toggle("small", entry.contentRect.width < 500);
}).observe(panel);
```
