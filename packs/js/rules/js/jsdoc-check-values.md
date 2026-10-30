---
id: js/jsdoc-check-values
title: "@version, @since and @kind take valid values"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the right version or kind is the author's"
stability: preview
ported_from: [eslint-plugin-jsdoc/check-values]
since: 0.5.0
---

`@version` and `@since` carry a semver version (`1.4.0`), and `@kind` one of JSDoc's kinds (`class`, `constant`, `event`, `external`, `file`, `function`, `member`, `mixin`, `module`, `namespace`, `typedef`). The upstream `@license` check against the SPDX list is left out.

**Why:** Documentation generators sort and filter by these values; one that is not a version or a kind is dropped or mis-sorted.

```codelaw
match:
  kind: comment
  all:
    - regex: '^/\*\*[^*/]'
    - any:
        - all:
            - regex: '(^/\*\*|\n)[ \t]*(\*[ \t]*)?@(version|since)\b'
            - not: {regex: '@(version|since)[ \t]+[v=]?\d+\.\d+\.\d+(-[0-9A-Za-z.-]+)?(\+[0-9A-Za-z.-]+)?([ \t\n]|\*/)'}
        - all:
            - regex: '(^/\*\*|\n)[ \t]*(\*[ \t]*)?@kind\b'
            - not: {regex: '@kind[ \t]+(class|constant|event|external|file|function|member|mixin|module|namespace|typedef)\b'}
message: "`@version` and `@since` take a semver version (`1.2.0`) and `@kind` one of JSDoc's kinds; correct the value"
```

## How to fix

1. Write the version as `major.minor.patch` (`@since 2.1.0`).
2. For `@kind`, use one of: class, constant, event, external, file, function, member, mixin, module, namespace, typedef.

## Examples

### JavaScript

#### Don't

```js
/**
 * Parses a date.
 * @since two
 */
function parseDate() {}
```

#### Do

```js
/**
 * Parses a date.
 * @since 2.1.0
 */
function parseDate() {}
```

### JSX

#### Don't

```jsx
/**
 * Parses a date.
 * @since two
 */
function parseDate() {}
```

#### Do

```jsx
/**
 * Parses a date.
 * @since 2.1.0
 */
function parseDate() {}
```

### TypeScript

#### Don't

```ts
/**
 * Parses a date.
 * @since two
 */
function parseDate() {}
```

#### Do

```ts
/**
 * Parses a date.
 * @since 2.1.0
 */
function parseDate() {}
```

### TSX

#### Don't

```tsx
/**
 * Parses a date.
 * @since two
 */
function parseDate() {}
```

#### Do

```tsx
/**
 * Parses a date.
 * @since 2.1.0
 */
function parseDate() {}
```
