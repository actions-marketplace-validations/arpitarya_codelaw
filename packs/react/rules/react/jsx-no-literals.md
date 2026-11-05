---
id: react/jsx-no-literals
title: "No bare text in JSX"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "value: the text moves into the project's message catalogue under a key the author chooses"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-react/jsx-no-literals]
since: 0.5.0
---

JSX holds no literal text between tags; text comes from an expression, such as a translation call.

**Why:** Bare strings in markup are what makes a UI hard to translate later; this rule is for projects that translate.

```codelaw
match:
  kind: jsx_text
  regex: '\S'
message: "Literal text in JSX; read it from the message catalogue"
```

## How to fix

1. Add the text to the project's message catalogue under a key.
2. Render it with the project's translation call, e.g. `{t("save")}`.

## Examples

### JavaScript

#### Don't

```js
const b = <button type="button">Save</button>;
```

#### Do

```js
const b = <button type="button">{t("save")}</button>;
```

### JSX

#### Don't

```jsx
const b = <button type="button">Save</button>;
```

#### Do

```jsx
const b = <button type="button">{t("save")}</button>;
```

### TSX

#### Don't

```tsx
const b = <button type="button">Save</button>;
```

#### Do

```tsx
const b = <button type="button">{t("save")}</button>;
```
