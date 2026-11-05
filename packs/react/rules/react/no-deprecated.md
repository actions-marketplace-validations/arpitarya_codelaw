---
id: react/no-deprecated
title: "No deprecated React APIs"
languages: [javascript, jsx, tsx]
fix: suggest
fix_reason: "the UNSAFE_ alias is exact for a legacy lifecycle, but the reviewer confirms React is 16.3 or later and nothing calls the method by name; the other APIs carry no edit"
stability: preview
ported_from: [eslint-plugin-react/no-deprecated]
since: 0.1.0
---

Code doesn't use React APIs that are deprecated: the legacy `componentWill…` lifecycles,
`React.createClass`, `React.PropTypes`, `ReactDOM.render`, `hydrate` and `unmountComponentAtNode`.

**Why:** Deprecated APIs are removed in later versions, and the legacy lifecycles misbehave with
concurrent rendering.

```codelaw
match:
  any:
    - kind: method_definition
      has: { field: name, regex: ^(componentWillMount|componentWillReceiveProps|componentWillUpdate)$, pattern: $M }
    - pattern: React.createClass($$$A)
    - pattern: React.PropTypes
    - pattern: ReactDOM.render($$$A)
    - pattern: ReactDOM.hydrate($$$A)
    - pattern: ReactDOM.unmountComponentAtNode($$$A)
    - pattern: ReactDOMServer.renderToNodeStream($$$A)
message: "This React API is deprecated and is removed in a later version"
fix:
  - replace: {target: $M, with: 'UNSAFE_$M'}
```

## How to fix

1. A legacy lifecycle takes its `UNSAFE_` name from the fix; the others are by hand. Replace `ReactDOM.render` with `createRoot(el).render(…)` and `hydrate` with `hydrateRoot`.
2. Rename legacy lifecycles to their `UNSAFE_` names, or move their work to `componentDidMount`/`componentDidUpdate`.
3. Use the `prop-types` package instead of `React.PropTypes`.

## Examples

### JavaScript

#### Don't

```js
class Card extends React.Component {
  componentWillMount() {
    this.load();
  }
}
```

#### Fixed

```js
class Card extends React.Component {
  UNSAFE_componentWillMount() {
    this.load();
  }
}
```

#### Do

```js
class Card extends React.Component {
  componentDidMount() {
    this.load();
  }
}
```

### JSX

#### Don't

```jsx
class Card extends React.Component {
  componentWillMount() {
    this.load();
  }
}
```

#### Fixed

```jsx
class Card extends React.Component {
  UNSAFE_componentWillMount() {
    this.load();
  }
}
```

#### Do

```jsx
class Card extends React.Component {
  componentDidMount() {
    this.load();
  }
}
```

### TSX

#### Don't

```tsx
class Card extends React.Component {
  componentWillMount() {
    this.load();
  }
}
```

#### Fixed

```tsx
class Card extends React.Component {
  UNSAFE_componentWillMount() {
    this.load();
  }
}
```

#### Do

```tsx
class Card extends React.Component {
  componentDidMount() {
    this.load();
  }
}
```

## Differences from the source

The React version setting is not read: every listed API is reported.
