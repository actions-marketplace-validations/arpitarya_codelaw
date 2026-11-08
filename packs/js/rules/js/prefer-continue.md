---
id: js/prefer-continue
title: "A `continue` guard instead of a loop body wrapped in one `if`"
languages: [javascript, jsx, typescript, tsx]
fix: auto
options: { maximumStatements: { type: "integer", default: 1, description: "report only when the if holds more statements than this" } }
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-continue]
since: 0.5.0
---

When a loop body ends with an `if` (no `else`) holding more than `maximumStatements` (1) statements that don't end in an exit, the `if` is reported and inverted: `if (!cond) { continue; }` followed by its statements, one level out. The condition is negated by dropping a `!`, flipping `===`/`!==`, or wrapping it in `!(…)`. A declaration inside the `if` when other statements come before it, a function or class declaration, a comment or a multi-line string inside, or a comment after it leave it without a fix. upstream's `checkShortBodies` mode is not taken.

**Why:** A guard that leaves early keeps the loop's main work at one level of indentation.

```codelaw
native: js/prefer-continue
```

## Examples

### JavaScript

#### Don't

```js
for (const job of jobs) {
  if (job.ready) {
    start(job);
    log(job);
  }
}
```

#### Do

```js
for (const job of jobs) {
  if (!job.ready) {
    continue;
  }

  start(job);
  log(job);
}
```

#### Fixed

```js
for (const job of jobs) {
  if (!job.ready) {
    continue;
  }

  start(job);
  log(job);
}
```


### JSX

#### Don't

```jsx
for (const job of jobs) {
  if (job.ready) {
    start(job);
    log(job);
  }
}
```

#### Do

```jsx
for (const job of jobs) {
  if (!job.ready) {
    continue;
  }

  start(job);
  log(job);
}
```

#### Fixed

```jsx
for (const job of jobs) {
  if (!job.ready) {
    continue;
  }

  start(job);
  log(job);
}
```


### TypeScript

#### Don't

```ts
for (const job of jobs) {
  if (job.ready) {
    start(job);
    log(job);
  }
}
```

#### Do

```ts
for (const job of jobs) {
  if (!job.ready) {
    continue;
  }

  start(job);
  log(job);
}
```

#### Fixed

```ts
for (const job of jobs) {
  if (!job.ready) {
    continue;
  }

  start(job);
  log(job);
}
```


### TSX

#### Don't

```tsx
for (const job of jobs) {
  if (job.ready) {
    start(job);
    log(job);
  }
}
```

#### Do

```tsx
for (const job of jobs) {
  if (!job.ready) {
    continue;
  }

  start(job);
  log(job);
}
```

#### Fixed

```tsx
for (const job of jobs) {
  if (!job.ready) {
    continue;
  }

  start(job);
  log(job);
}
```
