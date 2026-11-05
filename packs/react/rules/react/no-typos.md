---
id: react/no-typos
title: "No miscased React class members"
languages: [javascript, jsx, tsx]
fix: suggest
fix_reason: "each miscased name has one spelling in React's own list; the reviewer confirms the member was meant to be that lifecycle or static"
stability: preview
ported_from: [eslint-plugin-react/no-typos]
since: 0.5.0
---

A component's static properties and lifecycle methods are spelled as React spells them: `propTypes`, not `PropTypes` or `proptypes`; `componentDidMount`, not `componentdidmount`.

**Why:** React silently ignores a miscased name, so the validation or the lifecycle code never runs.

```codelaw
match:
  all:
    - kind: property_identifier
    - regex: '^(?i:proptypes|contexttypes|childcontexttypes|defaultprops|displayname|getderivedstatefromprops|componentdidmount|componentdidupdate|componentwillunmount|shouldcomponentupdate|getsnapshotbeforeupdate|componentdidcatch|getderivedstatefromerror|componentwillmount|componentwillreceiveprops|componentwillupdate)$'
    - not: { regex: '^(propTypes|contextTypes|childContextTypes|defaultProps|displayName|getDerivedStateFromProps|componentDidMount|componentDidUpdate|componentWillUnmount|shouldComponentUpdate|getSnapshotBeforeUpdate|componentDidCatch|getDerivedStateFromError|componentWillMount|componentWillReceiveProps|componentWillUpdate)$' }
    - any:
      - inside: { any: [{ kind: method_definition }, { kind: field_definition }, { kind: public_field_definition }], inside: { kind: class_body, inside: { kind: class_declaration, has: { kind: class_heritage, regex: '\b(Pure)?Component\b' } } } }
      - inside: { kind: member_expression, regex: '^[A-Z]\w*\.\w+$', inside: { kind: assignment_expression, field: left } }
    - any:
        - { kind: property_identifier, regex: '^(?i:proptypes)$', pattern: $T0 }
        - { kind: property_identifier, regex: '^(?i:contexttypes)$', pattern: $T1 }
        - { kind: property_identifier, regex: '^(?i:childcontexttypes)$', pattern: $T2 }
        - { kind: property_identifier, regex: '^(?i:defaultprops)$', pattern: $T3 }
        - { kind: property_identifier, regex: '^(?i:displayname)$', pattern: $T4 }
        - { kind: property_identifier, regex: '^(?i:getderivedstatefromprops)$', pattern: $T5 }
        - { kind: property_identifier, regex: '^(?i:componentdidmount)$', pattern: $T6 }
        - { kind: property_identifier, regex: '^(?i:componentdidupdate)$', pattern: $T7 }
        - { kind: property_identifier, regex: '^(?i:componentwillunmount)$', pattern: $T8 }
        - { kind: property_identifier, regex: '^(?i:shouldcomponentupdate)$', pattern: $T9 }
        - { kind: property_identifier, regex: '^(?i:getsnapshotbeforeupdate)$', pattern: $T10 }
        - { kind: property_identifier, regex: '^(?i:componentdidcatch)$', pattern: $T11 }
        - { kind: property_identifier, regex: '^(?i:getderivedstatefromerror)$', pattern: $T12 }
        - { kind: property_identifier, regex: '^(?i:componentwillmount)$', pattern: $T13 }
        - { kind: property_identifier, regex: '^(?i:componentwillreceiveprops)$', pattern: $T14 }
        - { kind: property_identifier, regex: '^(?i:componentwillupdate)$', pattern: $T15 }
message: "`$$MATCH` is miscased; React reads the camel-cased name"
fix:
  - replace: {target: $T0, with: 'propTypes'}
  - replace: {target: $T1, with: 'contextTypes'}
  - replace: {target: $T2, with: 'childContextTypes'}
  - replace: {target: $T3, with: 'defaultProps'}
  - replace: {target: $T4, with: 'displayName'}
  - replace: {target: $T5, with: 'getDerivedStateFromProps'}
  - replace: {target: $T6, with: 'componentDidMount'}
  - replace: {target: $T7, with: 'componentDidUpdate'}
  - replace: {target: $T8, with: 'componentWillUnmount'}
  - replace: {target: $T9, with: 'shouldComponentUpdate'}
  - replace: {target: $T10, with: 'getSnapshotBeforeUpdate'}
  - replace: {target: $T11, with: 'componentDidCatch'}
  - replace: {target: $T12, with: 'getDerivedStateFromError'}
  - replace: {target: $T13, with: 'componentWillMount'}
  - replace: {target: $T14, with: 'componentWillReceiveProps'}
  - replace: {target: $T15, with: 'componentWillUpdate'}
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

#### Fixed

```js
class A extends Component {
  componentDidMount() {}
}
A.propTypes = {};
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

#### Fixed

```jsx
class A extends Component {
  componentDidMount() {}
}
A.propTypes = {};
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

#### Fixed

```tsx
class A extends Component {
  componentDidMount() {}
}
A.propTypes = {};
```

#### Do

```tsx
class A extends Component {
  componentDidMount() {}
}
A.propTypes = {};
```
