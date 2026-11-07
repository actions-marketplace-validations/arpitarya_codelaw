---
id: js/no-unsafe-string-replacement
title: "A replacement function, not a computed string, in replace()"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "a value that was meant to use `$1` or `$&` loses that meaning in a function; the edit is offered for review"
stability: preview
ported_from: [eslint-plugin-unicorn/no-unsafe-string-replacement]
since: 0.5.0
---

`replace` and `replaceAll` read `$&`, `$1`, `$<name>` and `$$` in a replacement *string* as patterns. A replacement computed at run time — a call, a variable, a concatenation — can carry a `$` from input and garble the output. Wrapping it in a function (`() => value`) inserts it as written. A literal string or a template without substitutions is left.

**Why:** Input containing `$&` is rare enough to pass every test and common enough to reach production.

```codelaw
match:
  kind: call_expression
  has: { field: function, kind: member_expression, has: { field: property, regex: '^(replace|replaceAll)$' } }
  all:
  - has:
     field: arguments
     all:
      - has: { nthChild: 1 }
      - has:
          nthChild: 2
          pattern: $R
          any:
            - kind: call_expression
            - kind: identifier
            - kind: member_expression
            - kind: binary_expression
            - kind: template_string
              has: { kind: template_substitution }
          not:
            any:
              - kind: identifier
                regex: '^(undefined)$'
      - not: { has: { nthChild: 3 } }
message: "Insert the computed replacement through a function"
fix:
  - replace: { target: $R, with: "() => $R" }
```

## Examples

### JavaScript

#### Don't

```js
const html = template.replace("{name}", escape(name));
```

#### Do

```js
const html = template.replace("{name}", () => escape(name));
```


### JSX

#### Don't

```jsx
const html = template.replace("{name}", escape(name));
```

#### Do

```jsx
const html = template.replace("{name}", () => escape(name));
```


### TypeScript

#### Don't

```ts
const html = template.replace("{name}", escape(name));
```

#### Do

```ts
const html = template.replace("{name}", () => escape(name));
```


### TSX

#### Don't

```tsx
const html = template.replace("{name}", escape(name));
```

#### Do

```tsx
const html = template.replace("{name}", () => escape(name));
```
