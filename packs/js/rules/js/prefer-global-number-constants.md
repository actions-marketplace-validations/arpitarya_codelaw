---
id: js/prefer-global-number-constants
title: "NaN and Infinity rather than Number.NaN and Number.POSITIVE_INFINITY"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-global-number-constants]
since: 0.5.0
---

`Number.NaN` is the global `NaN` and `Number.POSITIVE_INFINITY` is the global `Infinity`; the short global names are used. `Number.NEGATIVE_INFINITY` is left: its global form is the expression `-Infinity`. The opposite convention is `js/prefer-number-properties`; turn on one of the two.

**Why:** The globals are the names every reader knows, and they are the same values.

```codelaw
match:
  any:
    - all:
        - pattern: Number.NaN
        - pattern: $W1
    - all:
        - pattern: Number.POSITIVE_INFINITY
        - pattern: $W2
  not:
    any:
      - inside: { kind: assignment_expression, field: left }
      - inside:
          stopBy: end
          kind: program
          has:
            stopBy: end
            any:
              - kind: variable_declarator
                has: { field: name, regex: '^(NaN|Infinity)$' }
              - kind: function_declaration
                has: { field: name, regex: '^(NaN|Infinity)$' }
              - kind: formal_parameters
                has: { stopBy: end, kind: identifier, regex: '^(NaN|Infinity)$' }
message: "Use the global constant"
fix:
  - replace: { target: $W1, with: NaN }
  - replace: { target: $W2, with: Infinity }
```

## Examples

### JavaScript

#### Don't

```js
const missing = Number.NaN;
const limit = Number.POSITIVE_INFINITY;
```

#### Do

```js
const missing = NaN;
const limit = Infinity;
```


### JSX

#### Don't

```jsx
const missing = Number.NaN;
const limit = Number.POSITIVE_INFINITY;
```

#### Do

```jsx
const missing = NaN;
const limit = Infinity;
```


### TypeScript

#### Don't

```ts
const missing = Number.NaN;
const limit = Number.POSITIVE_INFINITY;
```

#### Do

```ts
const missing = NaN;
const limit = Infinity;
```


### TSX

#### Don't

```tsx
const missing = Number.NaN;
const limit = Number.POSITIVE_INFINITY;
```

#### Do

```tsx
const missing = NaN;
const limit = Infinity;
```
