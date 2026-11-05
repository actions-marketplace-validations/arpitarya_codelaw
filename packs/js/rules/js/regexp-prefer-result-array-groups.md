---
id: js/regexp-prefer-result-array-groups
title: "A named group is read from `groups`"
languages: [javascript, jsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-regexp/prefer-result-array-groups]
since: 0.5.0
---

A match result's named group is read as `m.groups.name`, not `m[n]`.

**Why:** The name survives a reordering of the groups.

```codelaw
native: js/regexp-prefer-result-array-groups
```

## Examples

### JavaScript

#### Don't

```js
const m = /(?<y>\d+)/.exec(s);
use(m[1]);
```

#### Do

```js
const m = /(?<y>\d+)/.exec(s);
use(m.groups.y);
```

### JSX

#### Don't

```jsx
const m = /(?<y>\d+)/.exec(s);
use(m[1]);
```

#### Do

```jsx
const m = /(?<y>\d+)/.exec(s);
use(m.groups.y);
```
