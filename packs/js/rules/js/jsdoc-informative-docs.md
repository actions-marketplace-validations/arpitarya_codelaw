---
id: js/jsdoc-informative-docs
title: "JSDoc text that adds something to the name it documents"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "what the comment should say is the author's"
fix_exception: authored
options: { useless_words: { type: "string[]", default: ["a", "an", "i", "in", "of", "s", "the"], description: "words ignored when looking for one that adds meaning" }, excluded_tags: { type: "string[]", default: [], description: "tags whose description is not checked" } }
stability: preview
ported_from: [eslint-plugin-jsdoc/informative-docs]
since: 0.5.0
---

A block's description, and a tag's description, says something the documented name does not: text whose every word is one of the name's words, or one of `a`, `an`, `i`, `in`, `of`, `s`, `the` (the plugin's default useless words), is reported — `/** The name. */` on `name`, `@param name the name`. Words are compared in lower case after splitting at case changes (`getUserName` is `get`, `user`, `name`); `an` and `our` count as `a`. The names are the identifiers of the code the block documents: the declaration, a method's class too, a variable's initializer. The plugin's `aliases`, `uselessWords` and `excludedTags` options are built as `useless_words` and `excluded_tags`; `aliases` is not.

**Why:** A comment that only repeats the name adds noise, and its presence makes a reader think the thing is documented.

```codelaw
native: js/jsdoc-informative-docs
```

## How to fix

1. Say what the name does not: why it exists, what it returns, a unit, a constraint — or delete the comment.

## Examples

### JavaScript

#### Don't

```js
/**
 * The user name.
 * @param {string} userName the user name
 */
function setUserName(userName) {}
```

#### Do

```js
/**
 * Renames the signed-in account.
 * @param {string} userName the new display name, at most 40 characters
 */
function setUserName(userName) {}
```

### JSX

#### Don't

```jsx
/**
 * The user name.
 * @param {string} userName the user name
 */
function setUserName(userName) {}
```

#### Do

```jsx
/**
 * Renames the signed-in account.
 * @param {string} userName the new display name, at most 40 characters
 */
function setUserName(userName) {}
```

### TypeScript

#### Don't

```ts
/**
 * The user name.
 * @param {string} userName the user name
 */
function setUserName(userName) {}
```

#### Do

```ts
/**
 * Renames the signed-in account.
 * @param {string} userName the new display name, at most 40 characters
 */
function setUserName(userName) {}
```

### TSX

#### Don't

```tsx
/**
 * The user name.
 * @param {string} userName the user name
 */
function setUserName(userName) {}
```

#### Do

```tsx
/**
 * Renames the signed-in account.
 * @param {string} userName the new display name, at most 40 characters
 */
function setUserName(userName) {}
```
