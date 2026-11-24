---
id: js/no-implicit-globals
title: "No globals created by a script"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "making the name local or global on purpose changes where it is visible"
fix_exception: restructure
stability: preview
options:
  script: { type: "boolean", default: false, description: "the files this applies to are classic scripts (a `[scope.<name>]` over them), where a top-level declaration is global" }
  lexicalBindings: { type: "boolean", default: false, description: "also report top-level `let`, `const` and `class`" }
ported_from: [eslint/no-implicit-globals]
since: 0.5.2
---

In a classic script (a browser `<script>` with no `type="module"`), a top-level `var`, a `var` in a top-level block and a function declaration become properties of the global object, and an assignment to a name nothing declares creates one. They are reported; with `lexicalBindings`, top-level `let`, `const` and `class` too. Behaviour of ESLint 10.13.0's rule, read and reimplemented. ESLint 10 reads a `.js` file as a module, where a top-level declaration is module-scoped and nothing leaks, so the rule does nothing by default there, and so it is here: the option `script` says the files in scope are scripts. A file with an `import` or `export` is a module whatever the option says. A name listed in a `/* exported name */` comment is not reported. Not read: strict mode (a `"use strict"` makes a leak an error at run time instead), `/* global */` comments, and upstream's two messages about redeclaring or assigning a read-only global.

Switch it on for the script files: `[scope.browser-scripts] files = ["public/**/*.js"]` with `rules = { "js/no-implicit-globals" = { severity = "error", options = { script = true } } }`. With no `script` set the rule is silent: it reports nothing, not even that the option is missing, and the engine does not run it on a single node. It is in no preset; a project switches it on by setting the option under `[rules]` (or in a `[scope.<name>]` for some files).

In `codelaw.toml`, under `[rules]`: `"js/no-implicit-globals" = { severity = "error", options = { script = true } }`.

**Why:** A script's globals collide with every other script on the page; a declaration wrapped in a function, or put on `window` on purpose, does not.

```codelaw
native: js/no-implicit-globals
```

## How to fix

1. Wrap the code in an immediately invoked function, or assign to `window` (or `globalThis`) where a global is meant.
2. Declare the name an assignment leaks.

## Examples

### JavaScript

#### Don't

```js options={"script":true}
var counter = 0;
function bump() {
  counter += 1;
}
```

#### Do

```js options={"script":true}
(function () {
  var counter = 0;
  function bump() {
    counter += 1;
  }
  window.bump = bump;
})();
```

### JSX

#### Don't

```jsx options={"script":true}
var counter = 0;
function bump() {
  counter += 1;
}
```

#### Do

```jsx options={"script":true}
(function () {
  var counter = 0;
  function bump() {
    counter += 1;
  }
  window.bump = bump;
})();
```

### TypeScript

#### Don't

```ts options={"script":true}
var counter = 0;
function bump() {
  counter += 1;
}
```

#### Do

```ts options={"script":true}
(function () {
  var counter = 0;
  function bump() {
    counter += 1;
  }
  window.bump = bump;
})();
```

### TSX

#### Don't

```tsx options={"script":true}
var counter = 0;
function bump() {
  counter += 1;
}
```

#### Do

```tsx options={"script":true}
(function () {
  var counter = 0;
  function bump() {
    counter += 1;
  }
  window.bump = bump;
})();
```
