---
id: js/jsdoc-no-undefined-types
title: "JSDoc types name something defined"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the definition or import is the author's"
fix_exception: value
options: { defined_types: { type: "string[]", default: [], description: "type names to treat as defined" }, globals: { type: "string[]", default: [], description: "global names to treat as defined" }, env: { type: "string[]", default: [], description: "environments whose globals are defined: browser, node, commonjs, worker, serviceworker, shared-node-browser, jest, mocha, vitest" } }
stability: preview
ported_from: [eslint-plugin-jsdoc/no-undefined-types]
since: 0.5.0
---

A name used in a tag's `{type}` — or in an `@augments` / `@extends` — is defined: declared in the file (a variable, function, class, interface, type alias, enum, namespace, import or type parameter, in any scope), named by a `@typedef`, `@callback`, `@class` or other tag that defines a namepath, by a `@template` or an `@import` of the file, by a `/* globals … */` comment, by the rule's `defined_types` or `globals` options, by a data table of an environment named in `env`, or built in: JavaScript's globals, TypeScript's lib types (`Partial`, `Record`, `Awaited`, …) and the type keywords. The root of a dotted name (`Foo.Bar`) has to be defined. A type parameter of a function type, and an `infer` name, count within the type. As in the plugin, the environment's globals (the DOM, Node) are not defined unless `env` says so. Not read: the namepaths of other tags (`@memberof`, `@fires`, …), `checkUsedTypedefs` and `markVariablesAsUsed`.

**Why:** A type that names nothing is read as `any` by TypeScript and as an error by every documentation tool, and it is usually a typo or a missing import.

```codelaw
native: js/jsdoc-no-undefined-types
```

## How to fix

1. Import or declare the type, or add a `@typedef` for it.
2. Name an environment in the rule's `env` option when the type is one of its globals.

## Examples

### JavaScript

#### Don't

```js
/**
 * @param {Usr} user
 */
function greet(user) {}
class User {}
```

#### Do

```js
/**
 * @param {User} user
 */
function greet(user) {}
class User {}
```

### JSX

#### Don't

```jsx
/**
 * @param {Usr} user
 */
function greet(user) {}
class User {}
```

#### Do

```jsx
/**
 * @param {User} user
 */
function greet(user) {}
class User {}
```

### TypeScript

#### Don't

```ts
/**
 * @param {Usr} user
 */
function greet(user) {}
class User {}
```

#### Do

```ts
/**
 * @param {User} user
 */
function greet(user) {}
class User {}
```

### TSX

#### Don't

```tsx
/**
 * @param {Usr} user
 */
function greet(user) {}
class User {}
```

#### Do

```tsx
/**
 * @param {User} user
 */
function greet(user) {}
class User {}
```
