---
id: testing/playwright-no-unsafe-references
title: "No outer variables in `page.evaluate()` and `page.addInitScript()`"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: auto
fix_reason: "auto when the callback takes no parameters and is the only argument, or already destructures the array it is passed; every other shape is reported without an edit"
stability: preview
ported_from: ["eslint-plugin-playwright/no-unsafe-references"]
since: 0.5.0
---

The function given to `page.evaluate()` or `page.addInitScript()` (and the same calls on a frame: a receiver named `page…`, `frame…`, `…Page` or `…Frame`) does not read a variable declared outside it.

**Why:** The function is serialised and run in the browser, where the test's variables do not exist: the reference throws there, or reads another variable. Values reach the browser as the function's argument.

```codelaw
native: testing/playwright-no-unsafe-references
```

## How to fix

1. Pass the value as the argument and read it from the parameter: `page.evaluate(([id]) => …, [id])`.


## Examples

### JavaScript

#### Don't

```js
test("reads the element", async ({ page }) => {
  const id = "main";
  await page.evaluate(() => document.getElementById(id));
});
```

#### Do

```js
test("reads the element", async ({ page }) => {
  const id = "main";
  await page.evaluate(([id]) => document.getElementById(id), [id]);
});
```

### JSX

#### Don't

```jsx
test("reads the element", async ({ page }) => {
  const id = "main";
  await page.evaluate(() => document.getElementById(id));
});
```

#### Do

```jsx
test("reads the element", async ({ page }) => {
  const id = "main";
  await page.evaluate(([id]) => document.getElementById(id), [id]);
});
```

### TypeScript

#### Don't

```ts
test("reads the element", async ({ page }) => {
  const id = "main";
  await page.evaluate(() => document.getElementById(id));
});
```

#### Do

```ts
test("reads the element", async ({ page }) => {
  const id = "main";
  await page.evaluate(([id]) => document.getElementById(id), [id]);
});
```

### TSX

#### Don't

```tsx
test("reads the element", async ({ page }) => {
  const id = "main";
  await page.evaluate(() => document.getElementById(id));
});
```

#### Do

```tsx
test("reads the element", async ({ page }) => {
  const id = "main";
  await page.evaluate(([id]) => document.getElementById(id), [id]);
});
```

## Differences from the source

- The edit is made for the two shapes whose result is certain (see `fix_reason`); upstream rewrites other shapes and can misalign an existing parameter and argument. Each name appears once in the passed array.
- A top-level binding counts as outside the callback (a module); type-only references are skipped, as upstream.
