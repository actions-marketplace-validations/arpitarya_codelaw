---
id: js/dynamically-constructed-templates
title: "No Pug template compiled from a value that is not a literal"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "whether the template source can carry input is a security review"
fix_exception: security
stability: preview
ported_from: [eslint-plugin-sonarjs/dynamically-constructed-templates]
since: 0.5.0
---

`pug.compile`, `pug.render` and `pug.compileClient` run their first argument as template code. Given a variable, a call, a concatenation or a template literal with substitutions, input that reaches it can run code on the server. A plain string literal is left; load templates from files with `pug.compileFile`.

**Why:** Template injection is code execution.

```codelaw
match:
  kind: call_expression
  all:
    - has: { field: function, regex: '^(pug|pugModule)\.(compile|render|compileClient)$' }
    - has:
        field: arguments
        has:
          nthChild: 1
          not:
            any:
              - kind: string
              - kind: template_string
                not: { has: { kind: template_substitution } }
message: "This template is built at run time; make sure input can't reach it"
```

## How to fix

1. Keep templates in files and compile them with `pug.compileFile(path)`.
2. Pass data as locals (`fn({ name })`), never into the template source.

## Examples

### JavaScript

#### Don't

```js
const render = pug.compile(userTemplate);
```

#### Do

```js
const render = pug.compileFile("views/profile.pug");
```


### JSX

#### Don't

```jsx
const render = pug.compile(userTemplate);
```

#### Do

```jsx
const render = pug.compileFile("views/profile.pug");
```


### TypeScript

#### Don't

```ts
const render = pug.compile(userTemplate);
```

#### Do

```ts
const render = pug.compileFile("views/profile.pug");
```


### TSX

#### Don't

```tsx
const render = pug.compile(userTemplate);
```

#### Do

```tsx
const render = pug.compileFile("views/profile.pug");
```
