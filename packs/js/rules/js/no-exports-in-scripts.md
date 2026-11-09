---
id: js/no-exports-in-scripts
title: "No exports in a file run as a script"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "where the exported code should live (a module the script imports) is the author's layout"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-unicorn/no-exports-in-scripts]
since: 0.5.0
---

A file whose first line starts with `#!` is a script run directly, so each `export` in it — `export default`, named, `export … from`, type-only, even `export {}` — is reported. `module.exports` is left alone.

**Why:** Nothing imports a script, so its exports are dead, and a module that imports it would run it.

```codelaw
native: js/no-exports-in-scripts
```

## How to fix

1. Move what is exported to a module, and import it from the script.

## Examples

### JavaScript

#### Don't

```js
#!/usr/bin/env node
export function run() {}
run();
```

#### Do

```js
#!/usr/bin/env node
function run() {}
run();
```


### JSX

#### Don't

```jsx
#!/usr/bin/env node
export function run() {}
run();
```

#### Do

```jsx
#!/usr/bin/env node
function run() {}
run();
```


### TypeScript

#### Don't

```ts
#!/usr/bin/env node
export function run() {}
run();
```

#### Do

```ts
#!/usr/bin/env node
function run() {}
run();
```


### TSX

#### Don't

```tsx
#!/usr/bin/env node
export function run() {}
run();
```

#### Do

```tsx
#!/usr/bin/env node
function run() {}
run();
```
