---
id: js/regexp-prefer-named-backreference
title: "A named group is referred to by name"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-regexp/prefer-named-backreference]
since: 0.5.0
---

A backreference to a named group uses `\k<name>`, not its number.

**Why:** The name survives a reordering of the groups.

```codelaw
native: js/regexp-prefer-named-backreference
```

## Examples

### JavaScript

#### Don't

```js
const re = /(?<q>['"])\1/;
```

#### Do

```js
const re = /(?<q>['"])\k<q>/;
```

### JSX

#### Don't

```jsx
const re = /(?<q>['"])\1/;
```

#### Do

```jsx
const re = /(?<q>['"])\k<q>/;
```

### TypeScript

#### Don't

```ts
const re = /(?<q>['"])\1/;
```

#### Do

```ts
const re = /(?<q>['"])\k<q>/;
```

### TSX

#### Don't

```tsx
const re = /(?<q>['"])\1/;
```

#### Do

```tsx
const re = /(?<q>['"])\k<q>/;
```
