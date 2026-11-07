---
id: js/synchronous-suite-callback
title: "A describe() callback is not async"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "tests declared after an `await` in the callback were never registered; making it synchronous registers them, and the suite may now run tests it skipped"
stability: preview
ported_from: [eslint-plugin-sonarjs/synchronous-suite-callback]
since: 0.5.0
---

`describe` collects tests synchronously. With an `async` callback, Jest warns or fails and Mocha ignores the returned promise; tests declared after an `await` are never registered. The `async` keyword is removed where the callback awaits nothing; a callback that does await needs its set-up moved into a hook, and is reported without a fix.

**Why:** Tests silently go missing from the run.

```codelaw
match:
  kind: arrow_function
  regex: '^async\b'
  inside: { kind: arguments, inside: { kind: call_expression, has: { field: function, regex: '^(describe|suite|context)(\.(only|skip))?$' } } }
  not: { has: { stopBy: { any: [{ kind: arrow_function }, { kind: function_expression }, { kind: function_declaration }] }, kind: await_expression } }
message: "Make the suite callback synchronous"
fix:
  - edit-text: { pattern: '^async\s*', replace: '' }
```

## Examples

### JavaScript

#### Don't

```js
describe("users", async () => {
  it("lists", () => {});
});
```

#### Do

```js
describe("users", () => {
  it("lists", () => {});
});
```


### JSX

#### Don't

```jsx
describe("users", async () => {
  it("lists", () => {});
});
```

#### Do

```jsx
describe("users", () => {
  it("lists", () => {});
});
```


### TypeScript

#### Don't

```ts
describe("users", async () => {
  it("lists", () => {});
});
```

#### Do

```ts
describe("users", () => {
  it("lists", () => {});
});
```


### TSX

#### Don't

```tsx
describe("users", async () => {
  it("lists", () => {});
});
```

#### Do

```tsx
describe("users", () => {
  it("lists", () => {});
});
```
