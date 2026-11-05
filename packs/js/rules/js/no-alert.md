---
id: js/no-alert
title: "No alert, confirm or prompt"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the replacement is the app's own dialog or a log line, chosen by the author"
fix_exception: value
stability: preview
ported_from: [eslint/no-alert]
since: 0.5.0
---

The browser's `alert`, `confirm` and `prompt` dialogs are not called.

**Why:** They block the page, cannot be styled, and are usually debugging left behind.

```codelaw
match:
  any:
    - pattern: $F($$$A)
    - pattern: window.$F($$$A)
    - pattern: globalThis.$F($$$A)
constraints:
  F: { regex: '^(alert|confirm|prompt)$' }
message: "Do not call `$F`; use the app's own dialog"
```

## How to fix

1. Replace the call with the app's dialog component, or remove it.

## Examples

### JavaScript

#### Don't

```js
alert('Saved');
```

#### Do

```js
toast('Saved');
```

### JSX

#### Don't

```jsx
alert('Saved');
```

#### Do

```jsx
toast('Saved');
```

### TypeScript

#### Don't

```ts
alert('Saved');
```

#### Do

```ts
toast('Saved');
```

### TSX

#### Don't

```tsx
alert('Saved');
```

#### Do

```tsx
toast('Saved');
```
