---
id: next/no-location-assign-relative-destination
title: "router.push, not location for in-app paths"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the router comes from a hook or import the author adds"
fix_exception: restructure
stability: preview
ported_from: ["@next/eslint-plugin-next/no-location-assign-relative-destination"]
since: 0.5.0
---

An in-app path is navigated to with the router, not by assigning to `location`.

**Why:** Assigning to `location` reloads the whole app instead of navigating on the client.

```codelaw
match:
  any:
    - pattern: window.location = $D
    - pattern: window.location.href = $D
    - pattern: location.href = $D
    - pattern: location = $D
    - pattern: window.location.assign($D)
    - pattern: location.assign($D)
constraints:
  D: { kind: string, regex: '^["'']/(?:[^/]|$)' }
message: "Navigate inside the app with the router, not `location`"
```

## How to fix

1. Use `router.push(path)` from `next/navigation` (or `next/router`).

## Examples

### JavaScript

#### Don't

```js
window.location.href = '/dashboard';
```

#### Do

```js
router.push('/dashboard');
```

### JSX

#### Don't

```jsx
window.location.href = '/dashboard';
```

#### Do

```jsx
router.push('/dashboard');
```

### TypeScript

#### Don't

```ts
window.location.href = '/dashboard';
```

#### Do

```ts
router.push('/dashboard');
```

### TSX

#### Don't

```tsx
window.location.href = '/dashboard';
```

#### Do

```tsx
router.push('/dashboard');
```
