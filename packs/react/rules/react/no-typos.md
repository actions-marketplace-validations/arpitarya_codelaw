---
id: react/no-typos
title: "No miscased React class members"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "value: the intended name is read off the message; there is no template that recases a name"
fix_pending: "suggest-choices"
stability: preview
ported_from: [eslint-plugin-react/no-typos]
since: 0.5.0
---

A component's static properties and lifecycle methods are spelled as React spells them: `propTypes`, not `PropTypes` or `proptypes`; `componentDidMount`, not `componentdidmount`.

**Why:** React silently ignores a miscased name, so the validation or the lifecycle code never runs.

```codelaw
match:
  kind: property_identifier
  regex: '^(?i:proptypes|contexttypes|childcontexttypes|defaultprops|displayname|getderivedstatefromprops|componentdidmount|componentdidupdate|componentwillunmount|shouldcomponentupdate|getsnapshotbeforeupdate|componentdidcatch|getderivedstatefromerror|componentwillmount|componentwillreceiveprops|componentwillupdate)$'
  not: { regex: '^(propTypes|contextTypes|childContextTypes|defaultProps|displayName|getDerivedStateFromProps|componentDidMount|componentDidUpdate|componentWillUnmount|shouldComponentUpdate|getSnapshotBeforeUpdate|componentDidCatch|getDerivedStateFromError|componentWillMount|componentWillReceiveProps|componentWillUpdate)$' }
  any:
    - inside: { any: [{ kind: method_definition }, { kind: field_definition }, { kind: public_field_definition }], inside: { kind: class_body, inside: { kind: class_declaration, has: { kind: class_heritage, regex: '\b(Pure)?Component\b' } } } }
    - inside: { kind: member_expression, regex: '^[A-Z]\w*\.\w+$', inside: { kind: assignment_expression, field: left } }
message: "`$$MATCH` is miscased; React reads the camel-cased name"
```

## How to fix

1. Spell the name as React does: `propTypes`, `defaultProps`, `contextTypes`, `componentDidMount`, …

## Examples

### JavaScript

#### Don't

```js
class A extends Component {
  componentdidmount() {}
}
A.PropTypes = {};
```

#### Do

```js
class A extends Component {
  componentDidMount() {}
}
A.propTypes = {};
```

### JSX

#### Don't

```jsx
class A extends Component {
  componentdidmount() {}
}
A.PropTypes = {};
```

#### Do

```jsx
class A extends Component {
  componentDidMount() {}
}
A.propTypes = {};
```

### TSX

#### Don't

```tsx
class A extends Component {
  componentdidmount() {}
}
A.PropTypes = {};
```

#### Do

```tsx
class A extends Component {
  componentDidMount() {}
}
A.propTypes = {};
```
