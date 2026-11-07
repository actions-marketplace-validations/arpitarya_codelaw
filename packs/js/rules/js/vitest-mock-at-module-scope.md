---
id: js/vitest-mock-at-module-scope
title: "vi.mock() at the top level of the file"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "moving the mock may change which tests see it; the author checks"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-sonarjs/vitest-mock-at-module-scope]
since: 0.5.0
---

`vi.mock()` is called at the top level of a test file, not inside a function, a `describe` or a hook.

**Why:** Vitest hoists `vi.mock` to the top of the file whatever its position, so one written inside a block misleads: it applies to every test.

```codelaw
match:
  pattern: "vi.mock($$$A)"
  not: { inside: { kind: expression_statement, inside: { kind: program } } }
message: "`vi.mock` is hoisted; write it at the top level"
```

## How to fix

1. Move the call to the top level of the file; use `vi.doMock` where a mock must apply to some tests only.

## Examples

### JavaScript

#### Don't

```js
describe("api", () => {
  vi.mock("./client");
});
```

#### Do

```js
vi.mock("./client");
describe("api", () => {});
```


### JSX

#### Don't

```jsx
describe("api", () => {
  vi.mock("./client");
});
```

#### Do

```jsx
vi.mock("./client");
describe("api", () => {});
```


### TypeScript

#### Don't

```ts
describe("api", () => {
  vi.mock("./client");
});
```

#### Do

```ts
vi.mock("./client");
describe("api", () => {});
```


### TSX

#### Don't

```tsx
describe("api", () => {
  vi.mock("./client");
});
```

#### Do

```tsx
vi.mock("./client");
describe("api", () => {});
```
