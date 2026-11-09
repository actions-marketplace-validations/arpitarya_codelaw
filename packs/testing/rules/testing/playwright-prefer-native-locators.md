---
id: testing/playwright-prefer-native-locators
title: "`getByRole`, `getByLabel` and friends over `locator('[attr=…]')`"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: suggest
fix_reason: "the native locator matches by accessible name or role, case-insensitively and as a substring, where the attribute selector is exact"
stability: preview
options: { testIdAttribute: { type: "string", default: "data-testid" } }
ported_from: ["eslint-plugin-playwright/prefer-native-locators"]
since: 0.5.0
---

`locator('[aria-label="x"]')`, `[role=…]`, `[placeholder=…]`, `[alt=…]`, `[title=…]` and `[<testIdAttribute>=…]` (`data-testid` unless the option says otherwise) are written as `getByLabel`, `getByRole`, `getByPlaceholder`, `getByAltText`, `getByTitle` and `getByTestId`.

**Why:** The native locators read like the page is used, survive markup changes that keep the accessible name, and give a clearer error when nothing matches.

```codelaw
native: testing/playwright-prefer-native-locators
```


## Examples

### JavaScript

#### Don't

```js
test("fills the name", async ({ page }) => {
  await page.locator('[placeholder="Name"]').fill("Ada");
});
```

#### Do

```js
test("fills the name", async ({ page }) => {
  await page.getByPlaceholder("Name").fill("Ada");
});
```

#### Fixed

```js
test("fills the name", async ({ page }) => {
  await page.getByPlaceholder("Name").fill("Ada");
});
```

### JSX

#### Don't

```jsx
test("fills the name", async ({ page }) => {
  await page.locator('[placeholder="Name"]').fill("Ada");
});
```

#### Do

```jsx
test("fills the name", async ({ page }) => {
  await page.getByPlaceholder("Name").fill("Ada");
});
```

#### Fixed

```jsx
test("fills the name", async ({ page }) => {
  await page.getByPlaceholder("Name").fill("Ada");
});
```

### TypeScript

#### Don't

```ts
test("fills the name", async ({ page }) => {
  await page.locator('[placeholder="Name"]').fill("Ada");
});
```

#### Do

```ts
test("fills the name", async ({ page }) => {
  await page.getByPlaceholder("Name").fill("Ada");
});
```

#### Fixed

```ts
test("fills the name", async ({ page }) => {
  await page.getByPlaceholder("Name").fill("Ada");
});
```

### TSX

#### Don't

```tsx
test("fills the name", async ({ page }) => {
  await page.locator('[placeholder="Name"]').fill("Ada");
});
```

#### Do

```tsx
test("fills the name", async ({ page }) => {
  await page.getByPlaceholder("Name").fill("Ada");
});
```

#### Fixed

```tsx
test("fills the name", async ({ page }) => {
  await page.getByPlaceholder("Name").fill("Ada");
});
```

## Differences from the source

- Only a selector that is exactly one `[attribute=value]` (quoted or not) is reported: `[role=button][name="x"]` and a value with brackets, quotes or backslashes are not, where upstream's pattern would capture them whole and produce a wrong replacement.
- The fix is made only when `locator` has one argument; upstream's replacement drops `{ has: … }` options. It is `suggest` rather than `auto` because the native locator is not an exact attribute match.
