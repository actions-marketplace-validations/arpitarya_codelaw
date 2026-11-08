---
id: js/regexp-no-unused-capturing-group
title: "No capturing group whose capture is never read"
languages: [javascript, jsx, typescript, tsx]
fix: auto
on_edit: false
fix_reason: "Deletes a name a draft may be about to use, so save and the agent hook leave it for codelaw fix."
stability: preview
ported_from: [eslint-plugin-regexp/no-unused-capturing-group]
since: 0.5.0
---

A regular expression used only to test, search or replace with a fixed string has no capturing groups: `/(a)b/.test(s)`.

**Why:** The group costs and suggests the capture matters.

```codelaw
native: js/regexp-no-unused-capturing-group
```

## Examples

### JavaScript

#### Don't

```js
/(a)b/.test(s);
```

#### Do

```js
/(?:a)b/.test(s);
```

### JSX

#### Don't

```jsx
/(a)b/.test(s);
```

#### Do

```jsx
/(?:a)b/.test(s);
```

### TypeScript

#### Don't

```ts
/(a)b/.test(s);
```

#### Do

```ts
/(?:a)b/.test(s);
```

### TSX

#### Don't

```tsx
/(a)b/.test(s);
```

#### Do

```tsx
/(?:a)b/.test(s);
```
