---
id: react/state-in-constructor
title: "State is set in the constructor"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "restructure: the field moves into a constructor, which may not exist yet"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-react/state-in-constructor]
since: 0.5.0
---

A class component sets its initial state in the constructor (`this.state = …`), not as a class field.

**Why:** One place for initial state; this is the style for codebases that write constructors.

```codelaw
match:
  any: [{ kind: field_definition }, { kind: public_field_definition }]
  regex: '^state\s*='
  inside: { kind: class_body, inside: { kind: class_declaration, has: { kind: class_heritage, regex: '\b(Pure)?Component\b' } } }
message: "Set the initial state in the constructor"
```

## How to fix

1. Write a constructor (calling `super(props)`) and move the value to `this.state = …` there.

## Examples

### JavaScript

#### Don't

```js
class C extends Component {
  state = { open: false };
}
```

#### Do

```js
class C extends Component {
  constructor(props) {
    super(props);
    this.state = { open: false };
  }
}
```

### JSX

#### Don't

```jsx
class C extends Component {
  state = { open: false };
}
```

#### Do

```jsx
class C extends Component {
  constructor(props) {
    super(props);
    this.state = { open: false };
  }
}
```

### TSX

#### Don't

```tsx
class C extends Component {
  state = { open: false };
}
```

#### Do

```tsx
class C extends Component {
  constructor(props) {
    super(props);
    this.state = { open: false };
  }
}
```
