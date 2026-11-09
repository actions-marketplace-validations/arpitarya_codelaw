---
id: testing/valid-mock-module-path
title: "`jest.mock` names a module that exists"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: manual
fix_reason: "the right path is the one the author meant; nothing in the file says which"
fix_exception: value
stability: preview
options: { moduleFileExtensions: { type: "string[]", default: [".js", ".ts", ".tsx", ".jsx", ".json"] } }
ported_from: ["eslint-plugin-jest/valid-mock-module-path"]
since: 0.5.0
---

`jest.mock('./path')` and `jest.doMock('./path')` name a relative path that exists: the file, the file plus one of `moduleFileExtensions`, or a folder. `{ virtual: true }` as the third argument says the module is made up, and is skipped.

**Why:** Jest does not fail on a mock of a module that is not there: the mock is registered and never used, so the test runs against the real module, or against nothing, and passes for the wrong reason. A rename or a move leaves the old path behind.

```codelaw
native: testing/valid-mock-module-path
```

## How to fix

1. Point the mock at the module's current path.
2. Or, for a module that does not exist on purpose, pass `{ virtual: true }` as the third argument.


## Examples

### JavaScript

#### Don't

```js
jest.mock("./does-not-exist");
```

#### Do

```js
jest.mock("./does-not-exist", () => ({}), { virtual: true });
```

### JSX

#### Don't

```jsx
jest.mock("./does-not-exist");
```

#### Do

```jsx
jest.mock("./does-not-exist", () => ({}), { virtual: true });
```

### TypeScript

#### Don't

```ts
jest.mock("./does-not-exist");
```

#### Do

```ts
jest.mock("./does-not-exist", () => ({}), { virtual: true });
```

### TSX

#### Don't

```tsx
jest.mock("./does-not-exist");
```

#### Do

```tsx
jest.mock("./does-not-exist", () => ({}), { virtual: true });
```

## Differences from the source

Only a relative path (`./`, `../`) is checked, against the project's files. Upstream also resolves a package name through Node (`require.resolve`), which the project's files do not give: `jest.mock('some-package')` is not reported. A path that leaves the project root is not checked.
