---
id: js/consistent-template-literal-escape
title: "A literal ${ in a template is written \\${"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/consistent-template-literal-escape]
since: 0.5.0
---

To put the characters `${` in a template literal without starting a substitution, escape the dollar sign: `\${`. Escaping the brace (`$\{`) or both (`\$\{`) gives the same string, so the rule rewrites them to the one form.

**Why:** Three spellings of the same text make a reader stop to check each; one is enough.

```codelaw
match:
  kind: template_string
  regex: '\$\\\{'
message: "Escape the dollar sign: write `\\${`"
fix:
  - edit-text: { pattern: '\\\$\\\{', replace: '\${' }
  - edit-text: { pattern: '\$\\\{', replace: '\${' }
```

## Examples

### JavaScript

#### Don't

```js
const hint = `Write $\{name} to insert the name`;
```

#### Do

```js
const hint = `Write \${name} to insert the name`;
```


### JSX

#### Don't

```jsx
const hint = `Write $\{name} to insert the name`;
```

#### Do

```jsx
const hint = `Write \${name} to insert the name`;
```


### TypeScript

#### Don't

```ts
const hint = `Write $\{name} to insert the name`;
```

#### Do

```ts
const hint = `Write \${name} to insert the name`;
```


### TSX

#### Don't

```tsx
const hint = `Write $\{name} to insert the name`;
```

#### Do

```tsx
const hint = `Write \${name} to insert the name`;
```
