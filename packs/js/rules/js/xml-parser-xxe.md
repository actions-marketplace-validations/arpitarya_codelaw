---
id: js/xml-parser-xxe
title: "No external entities in XML parsing"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "a document that needs entities must be trusted first"
stability: preview
ported_from: [eslint-plugin-sonarjs/xml-parser-xxe]
since: 0.5.0
---

An XML parser is not given `noent: true`.

**Why:** Entity expansion lets a document pull in files and URLs (XML external entities).

```codelaw
match:
  kind: pair
  has:
    field: key
    regex: ^(noent|"noent")$
  all:
  - has:
      field: value
      kind: 'true'
message: Expanding entities lets an XML document read local files (XXE)
```

## How to fix

1. Remove `noent: true`; parse only trusted documents if entities are required.

## Examples

### JavaScript

#### Don't

```js
const doc = libxmljs.parseXml(xml, { noent: true });
```

#### Do

```js
const doc = libxmljs.parseXml(xml);
```

### JSX

#### Don't

```jsx
const doc = libxmljs.parseXml(xml, { noent: true });
```

#### Do

```jsx
const doc = libxmljs.parseXml(xml);
```

### TypeScript

#### Don't

```ts
const doc = libxmljs.parseXml(xml, { noent: true });
```

#### Do

```ts
const doc = libxmljs.parseXml(xml);
```

### TSX

#### Don't

```tsx
const doc = libxmljs.parseXml(xml, { noent: true });
```

#### Do

```tsx
const doc = libxmljs.parseXml(xml);
```
