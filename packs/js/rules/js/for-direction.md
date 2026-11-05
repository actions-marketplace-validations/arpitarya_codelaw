---
id: js/for-direction
title: "A for loop's counter moves toward its end condition"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "whether the update or the comparison is the wrong half needs the author; both flips are offered"
stability: preview
ported_from: [eslint/for-direction]
since: 0.1.0
---

A `for` loop whose condition tests `i < n` counts up, and one that tests `i > n` counts down.

**Why:** A counter that moves the wrong way never meets its end condition, so the loop never ends.

```codelaw
match:
  kind: for_statement
  any:
    - all:
        - has:
            field: condition
            any:
              - { pattern: $I < $END, all: [{ pattern: $LT }] }
              - { pattern: $I <= $END, all: [{ pattern: $LE }] }
        - has:
            field: increment
            any:
              - { pattern: $I--, all: [{ pattern: $POSTDEC }] }
              - { pattern: --$I, all: [{ pattern: $PREDEC }] }
    - all:
        - has:
            field: condition
            any:
              - { pattern: $I > $END, all: [{ pattern: $GT }] }
              - { pattern: $I >= $END, all: [{ pattern: $GE }] }
        - has:
            field: increment
            any:
              - { pattern: $I++, all: [{ pattern: $POSTINC }] }
              - { pattern: ++$I, all: [{ pattern: $PREINC }] }
message: "The counter `$I` moves away from the condition that ends this loop"
fix:
  choices:
    - label: flip the update
      default: true
      fix:
        - { replace: { target: $POSTDEC, with: "$I++" } }
        - { replace: { target: $PREDEC, with: "++$I" } }
        - { replace: { target: $POSTINC, with: "$I--" } }
        - { replace: { target: $PREINC, with: "--$I" } }
    - label: flip the comparison
      fix:
        - { replace: { target: $LT, with: "$I > $END" } }
        - { replace: { target: $LE, with: "$I >= $END" } }
        - { replace: { target: $GT, with: "$I < $END" } }
        - { replace: { target: $GE, with: "$I <= $END" } }
```

## How to fix

1. Decide which way the loop should count.
2. Fix the update (`i++` or `i--`) or the comparison to match.

## Examples

### JavaScript

#### Don't

```js
for (let i = 0; i < 10; i--) {
  draw(i);
}
```

#### Fixed (flip the update)

```js
for (let i = 0; i < 10; i++) {
  draw(i);
}
```

#### Fixed (flip the comparison)

```js
for (let i = 0; i > 10; i--) {
  draw(i);
}
```

#### Do

```js
for (let i = 0; i < 10; i++) {
  draw(i);
}
```

### JSX

#### Don't

```jsx
for (let i = 0; i < 10; i--) {
  draw(i);
}
```

#### Fixed (flip the update)

```jsx
for (let i = 0; i < 10; i++) {
  draw(i);
}
```

#### Fixed (flip the comparison)

```jsx
for (let i = 0; i > 10; i--) {
  draw(i);
}
```

#### Do

```jsx
for (let i = 0; i < 10; i++) {
  draw(i);
}
```

### TypeScript

#### Don't

```ts
for (let i = 0; i < 10; i--) {
  draw(i);
}
```

#### Fixed (flip the update)

```ts
for (let i = 0; i < 10; i++) {
  draw(i);
}
```

#### Fixed (flip the comparison)

```ts
for (let i = 0; i > 10; i--) {
  draw(i);
}
```

#### Do

```ts
for (let i = 0; i < 10; i++) {
  draw(i);
}
```

### TSX

#### Don't

```tsx
for (let i = 0; i < 10; i--) {
  draw(i);
}
```

#### Fixed (flip the update)

```tsx
for (let i = 0; i < 10; i++) {
  draw(i);
}
```

#### Fixed (flip the comparison)

```tsx
for (let i = 0; i > 10; i--) {
  draw(i);
}
```

#### Do

```tsx
for (let i = 0; i < 10; i++) {
  draw(i);
}
```
