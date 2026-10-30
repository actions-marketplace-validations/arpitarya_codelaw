---
id: react/static-property-placement
title: "Static component properties are assigned after the class"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "restructure: the property moves out of the class body to an assignment after it"
stability: preview
ported_from: [eslint-plugin-react/static-property-placement]
since: 0.5.0
---

`propTypes`, `defaultProps`, `contextTypes`, `childContextTypes`, `contextType` and `displayName` are assigned after the class (`Card.propTypes = …`), not declared `static` inside it.

**Why:** One place for them; this is the style for codebases that keep the class body for behaviour.

```codelaw
match:
  any: [{ kind: field_definition }, { kind: public_field_definition }]
  regex: '^static\s+(propTypes|defaultProps|contextTypes|childContextTypes|contextType|displayName)\b'
  inside: { kind: class_body, inside: { kind: class_declaration, has: { kind: class_heritage, regex: '\b(Pure)?Component\b' } } }
message: "Assign this after the class instead of declaring it `static`"
```

## How to fix

1. Delete the static field and assign it after the class: `Card.propTypes = {…};`.

## Examples

### JavaScript

#### Don't

```js
class Card extends Component {
  static displayName = "Card";
}
```

#### Do

```js
class Card extends Component {}
Card.displayName = "Card";
```

### JSX

#### Don't

```jsx
class Card extends Component {
  static displayName = "Card";
}
```

#### Do

```jsx
class Card extends Component {}
Card.displayName = "Card";
```

### TSX

#### Don't

```tsx
class Card extends Component {
  static displayName = "Card";
}
```

#### Do

```tsx
class Card extends Component {}
Card.displayName = "Card";
```
