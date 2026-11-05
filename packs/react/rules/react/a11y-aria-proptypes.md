---
id: react/a11y-aria-proptypes
title: "ARIA attribute values have the right type"
languages: [javascript, jsx, tsx]
fix: suggest
fix_reason: "for the true/false-typed attributes the reviewer picks `true` or `false`; attributes that take other tokens or numbers get no edit"
stability: preview
ported_from: [eslint-plugin-jsx-a11y/aria-proptypes]
since: 0.5.0
---

An `aria-*` attribute written as a string holds a value its type allows: `true`/`false` for a boolean, one of its tokens for a token, an integer for a count.

**Why:** An invalid value is ignored, or read as the default, by assistive technology.

```codelaw
match:
  kind: jsx_attribute
  any:
    - regex: '^aria-(atomic|busy|disabled|modal|multiline|multiselectable|readonly|required)=["'']'
      has: {kind: string, pattern: $V}
      not: { regex: '^aria-[a-z]+=(["''])(true|false)["'']$' }
    - regex: '^aria-(checked|pressed)=["'']'
      has: {kind: string, pattern: $V}
      not: { regex: '^aria-[a-z]+=(["''])(true|false|mixed|undefined)["'']$' }
    - regex: '^aria-(expanded|grabbed|hidden|selected)=["'']'
      has: {kind: string, pattern: $V}
      not: { regex: '^aria-[a-z]+=(["''])(true|false|undefined)["'']$' }
    - regex: '^aria-autocomplete=["'']'
      not: { regex: '^aria-[a-z]+=(["''])(inline|list|both|none)["'']$' }
    - regex: '^aria-current=["'']'
      not: { regex: '^aria-[a-z]+=(["''])(page|step|location|date|time|true|false)["'']$' }
    - regex: '^aria-haspopup=["'']'
      not: { regex: '^aria-[a-z]+=(["''])(false|true|menu|listbox|tree|grid|dialog)["'']$' }
    - regex: '^aria-invalid=["'']'
      not: { regex: '^aria-[a-z]+=(["''])(grammar|false|spelling|true)["'']$' }
    - regex: '^aria-live=["'']'
      not: { regex: '^aria-[a-z]+=(["''])(assertive|off|polite)["'']$' }
    - regex: '^aria-orientation=["'']'
      not: { regex: '^aria-[a-z]+=(["''])(vertical|horizontal|undefined)["'']$' }
    - regex: '^aria-sort=["'']'
      not: { regex: '^aria-[a-z]+=(["''])(ascending|descending|none|other)["'']$' }
    - regex: '^aria-(level|posinset|setsize|colcount|colindex|colspan|rowcount|rowindex|rowspan)=["'']'
      not: { regex: '^aria-[a-z]+=(["''])-?[0-9]+["'']$' }
message: "`$$MATCH` is not a value this ARIA attribute takes"
fix:
  choices:
    - label: "true"
      fix:
        - replace: { target: $V, with: '"true"' }
    - label: "false"
      fix:
        - replace: { target: $V, with: '"false"' }
```

## How to fix

1. Use a value the attribute's type allows (for example `aria-hidden="true"`).

## Examples

### JavaScript

#### Don't

```js
const b = <button aria-pressed="yes">Bold</button>;
```

#### Fixed (true)

```js
const b = <button aria-pressed="true">Bold</button>;
```

#### Fixed (false)

```js
const b = <button aria-pressed="false">Bold</button>;
```

#### Do

```js
const b = <button aria-pressed="true">Bold</button>;
```

### JSX

#### Don't

```jsx
const b = <button aria-pressed="yes">Bold</button>;
```

#### Fixed (true)

```jsx
const b = <button aria-pressed="true">Bold</button>;
```

#### Fixed (false)

```jsx
const b = <button aria-pressed="false">Bold</button>;
```

#### Do

```jsx
const b = <button aria-pressed="true">Bold</button>;
```

### TSX

#### Don't

```tsx
const b = <button aria-pressed="yes">Bold</button>;
```

#### Fixed (true)

```tsx
const b = <button aria-pressed="true">Bold</button>;
```

#### Fixed (false)

```tsx
const b = <button aria-pressed="false">Bold</button>;
```

#### Do

```tsx
const b = <button aria-pressed="true">Bold</button>;
```

## Differences from the source

Values written as expressions (`aria-hidden={x}`) are not checked; the source also checks literal expressions.
