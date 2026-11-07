---
id: js/no-invalid-style-set-property
title: "setProperty() takes a CSS property name and a separate priority"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the kebab-case name is derived from the camelCase one; a custom name that only looks camelCase needs review"
stability: preview
ported_from: [eslint-plugin-unicorn/no-invalid-style-set-property]
since: 0.5.0
---

`style.setProperty(name, value, priority)` takes the CSS name (`'background-color'`, not `'backgroundColor'`), and `!important` goes in the third argument as `'important'`, not in the value or with its `!`. Each of these is silently ignored by the browser. Custom properties (`--brandColor`) keep their case.

**Why:** The call does nothing and gives no error, so the style just doesn't apply.

```codelaw
match:
  kind: call_expression
  has: { field: function, kind: member_expression, has: { field: property, regex: '^setProperty$' } }
  all:
  - has:
     field: arguments
     any:
      - has: { nthChild: 1, kind: string, regex: '^["''][a-z]+([A-Z][a-z]*)+["'']$', has: { kind: string_fragment, pattern: $NAME } }
      - all:
          - has: { nthChild: 2, kind: string, regex: '\s!important["'']$', has: { kind: string_fragment, pattern: $VALUE } }
          - not: { has: { nthChild: 3 } }
          - has: { nthChild: 2, pattern: $VALSTR }
      - has: { nthChild: 3, kind: string, regex: '^["'']!important["'']$', pattern: $PRIO }
transform:
  KEBAB: { convert: { source: $NAME, toCase: kebabCase } }
  BARE: { replace: { source: $VALUE, replace: '\s*!important$', by: '' } }
message: "`setProperty` ignores this argument as written"
fix:
  - replace: { target: $NAME, with: $KEBAB }
  - replace: { target: $VALSTR, with: "\"$BARE\", \"important\"" }
  - replace: { target: $PRIO, with: "\"important\"" }
```

## Examples

### JavaScript

#### Don't

```js
element.style.setProperty("backgroundColor", "red");
```

#### Do

```js
element.style.setProperty("background-color", "red");
```


### JSX

#### Don't

```jsx
element.style.setProperty("backgroundColor", "red");
```

#### Do

```jsx
element.style.setProperty("background-color", "red");
```


### TypeScript

#### Don't

```ts
element.style.setProperty("backgroundColor", "red");
```

#### Do

```ts
element.style.setProperty("background-color", "red");
```


### TSX

#### Don't

```tsx
element.style.setProperty("backgroundColor", "red");
```

#### Do

```tsx
element.style.setProperty("background-color", "red");
```
