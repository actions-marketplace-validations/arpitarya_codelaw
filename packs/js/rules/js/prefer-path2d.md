---
id: js/prefer-path2d
title: "A Path2D built once rather than the same path rebuilt every frame"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "which parts of the path stay the same between frames is the author's to say"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-path2d]
since: 0.5.0
---

A function passed to `requestAnimationFrame` or `setInterval`, or a loop body, that builds a path on a canvas context (`moveTo`, `lineTo`, `arc`, `rect`, the curve methods) and then calls `stroke()` or `fill()` on it repeats the path every time. A `Path2D` built once and passed to `stroke(path)` or `fill(path)` does that work once. Upstream also follows local functions called from such places; those are not reported here.

**Why:** Rebuilding a constant path every frame costs time for nothing.

```codelaw
utils:
  repeated:
    any:
      - kind: for_statement
      - kind: for_in_statement
      - kind: while_statement
      - all:
          - any: [{ kind: arrow_function }, { kind: function_expression }]
          - inside: { kind: arguments, inside: { kind: call_expression, has: { field: function, regex: '^(window\.)?(requestAnimationFrame|setInterval)$' } } }
match:
  kind: expression_statement
  any:
    - pattern: "$C.stroke();"
    - pattern: "$C.fill();"
  inside: { stopBy: end, matches: repeated }
  follows:
    stopBy: end
    kind: expression_statement
    has: { kind: call_expression, pattern: "$C.$M($$$)", has: { field: function, has: { field: property, regex: '^(moveTo|lineTo|arc|arcTo|rect|bezierCurveTo|quadraticCurveTo|ellipse)$' } } }
message: "Build this path once with `Path2D`"
```

## How to fix

1. Build `const path = new Path2D()` once, with the same `moveTo`/`arc`/… calls, outside the loop or frame callback.
2. Draw it each time with `context.stroke(path)` or `context.fill(path)`.

## Examples

### JavaScript

#### Don't

```js
requestAnimationFrame(() => {
  context.moveTo(220, 60);
  context.arc(170, 60, 50, 0, 2 * Math.PI);
  context.stroke();
});
```

#### Do

```js
const circle = new Path2D();
circle.arc(170, 60, 50, 0, 2 * Math.PI);
requestAnimationFrame(() => {
  context.stroke(circle);
});
```


### JSX

#### Don't

```jsx
requestAnimationFrame(() => {
  context.moveTo(220, 60);
  context.arc(170, 60, 50, 0, 2 * Math.PI);
  context.stroke();
});
```

#### Do

```jsx
const circle = new Path2D();
circle.arc(170, 60, 50, 0, 2 * Math.PI);
requestAnimationFrame(() => {
  context.stroke(circle);
});
```


### TypeScript

#### Don't

```ts
requestAnimationFrame(() => {
  context.moveTo(220, 60);
  context.arc(170, 60, 50, 0, 2 * Math.PI);
  context.stroke();
});
```

#### Do

```ts
const circle = new Path2D();
circle.arc(170, 60, 50, 0, 2 * Math.PI);
requestAnimationFrame(() => {
  context.stroke(circle);
});
```


### TSX

#### Don't

```tsx
requestAnimationFrame(() => {
  context.moveTo(220, 60);
  context.arc(170, 60, 50, 0, 2 * Math.PI);
  context.stroke();
});
```

#### Do

```tsx
const circle = new Path2D();
circle.arc(170, 60, 50, 0, 2 * Math.PI);
requestAnimationFrame(() => {
  context.stroke(circle);
});
```
