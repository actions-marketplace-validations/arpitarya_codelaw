---
id: react/no-is-mounted
title: "No this.isMounted()"
languages: [javascript, jsx, tsx]
fix: suggest
fix_reason: "a guard with only a block of statements and no else is dropped, keeping the block (a setState after unmount is a no-op); the reviewer confirms the callback needs no cancellation instead"
stability: preview
ported_from: [eslint-plugin-react/no-is-mounted]
since: 0.1.0
---

Code doesn't call `this.isMounted()`.

**Why:** `isMounted` hides leaks: a callback that outlives the component should be cancelled
instead.

```codelaw
match:
  any:
    - pattern: this.isMounted()
      inside:
        stopBy: end
        all:
          - pattern: $IF
          - pattern: 'if (this.isMounted()) { $$$B }'
    - pattern: this.isMounted()
message: "`isMounted` is an anti-pattern and gone from ES6 classes"
fix:
  - replace: {target: $IF, with: '$$$B'}
```

## How to fix

1. Cancel pending work in `componentWillUnmount`.
2. Remove the `isMounted` check.

## Examples

### JavaScript

#### Don't

```js
class A extends React.Component {
  done() {
    if (this.isMounted()) {
      this.setState({ ok: true });
    }
  }
}
```

#### Fixed

```js
class A extends React.Component {
  done() {
    this.setState({ ok: true });
  }
}
```

#### Do

```js
class A extends React.Component {
  componentWillUnmount() {
    this.request.cancel();
  }
  done() {
    this.setState({ ok: true });
  }
}
```

### JSX

#### Don't

```jsx
class A extends React.Component {
  done() {
    if (this.isMounted()) {
      this.setState({ ok: true });
    }
  }
}
```

#### Fixed

```jsx
class A extends React.Component {
  done() {
    this.setState({ ok: true });
  }
}
```

#### Do

```jsx
class A extends React.Component {
  componentWillUnmount() {
    this.request.cancel();
  }
  done() {
    this.setState({ ok: true });
  }
}
```

### TSX

#### Don't

```tsx
class A extends React.Component {
  done() {
    if (this.isMounted()) {
      this.setState({ ok: true });
    }
  }
}
```

#### Fixed

```tsx
class A extends React.Component {
  done() {
    this.setState({ ok: true });
  }
}
```

#### Do

```tsx
class A extends React.Component {
  componentWillUnmount() {
    this.request.cancel();
  }
  done() {
    this.setState({ ok: true });
  }
}
```
