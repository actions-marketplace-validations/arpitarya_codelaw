---
id: js/template-indent
title: "Indent a whitespace-insensitive template to match its code"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "the template's lines are shifted as a block, which cannot change code; when a template's whitespace matters to the program it is not one the rule is told about, so it is not touched"
options: { tags: { type: "string[]", default: ["outdent", "dedent", "gql", "sql", "html", "styled"], description: "tags of the templates to indent" }, functions: { type: "string[]", default: ["dedent", "stripIndent"], description: "functions whose template argument is indented" }, comments: { type: "string[]", default: ["HTML", "indent"], description: "block comments before a template that mark it, as `/* HTML */`" }, indent: { type: "string", default: "", description: "the indentation inside the template, as whitespace or a number of spaces; empty reads the file" } }
stability: preview
ported_from: [eslint-plugin-unicorn/template-indent]
since: 0.5.0
---

A multi-line template literal that upstream is told is whitespace-insensitive is re-indented to sit one step inside the line it starts on: a tagged template whose tag is one of `tags`, the template argument of a call to one of `functions`, a template right after a `/* HTML */` or `/* indent */` comment, and the argument of Jest's `toMatchInlineSnapshot()` and `toThrowErrorMatchingInlineSnapshot()`. The common indentation of the lines is removed, then each line gets the margin of the starting line plus one indentation step: the file's own step (read from its code, outside templates), else the template's, else a tab or two spaces by the starting line. The relative indentation of the lines is kept, and `${…}` substitutions are left in place. A template with text or a substitution on its first line, or without a line break, is left alone. Upstream's `selectors` option (any ESLint selector) is not read.

**Why:** A template whose whitespace does not matter should look like the code around it, not hang at the left margin or float at a stale indent.

```codelaw
native: js/template-indent
```

## Examples

### JavaScript

#### Don't

```js
function run() {
  const query = sql`
select *
from students
`;
  return query;
}
```

#### Do

```js
function run() {
  const query = sql`
    select *
    from students
  `;
  return query;
}
```

#### Fixed

```js
function run() {
  const query = sql`
    select *
    from students
  `;
  return query;
}
```


### JSX

#### Don't

```jsx
function run() {
  const query = sql`
select *
from students
`;
  return query;
}
```

#### Do

```jsx
function run() {
  const query = sql`
    select *
    from students
  `;
  return query;
}
```

#### Fixed

```jsx
function run() {
  const query = sql`
    select *
    from students
  `;
  return query;
}
```


### TypeScript

#### Don't

```ts
function run() {
  const query = sql`
select *
from students
`;
  return query;
}
```

#### Do

```ts
function run() {
  const query = sql`
    select *
    from students
  `;
  return query;
}
```

#### Fixed

```ts
function run() {
  const query = sql`
    select *
    from students
  `;
  return query;
}
```


### TSX

#### Don't

```tsx
function run() {
  const query = sql`
select *
from students
`;
  return query;
}
```

#### Do

```tsx
function run() {
  const query = sql`
    select *
    from students
  `;
  return query;
}
```

#### Fixed

```tsx
function run() {
  const query = sql`
    select *
    from students
  `;
  return query;
}
```
