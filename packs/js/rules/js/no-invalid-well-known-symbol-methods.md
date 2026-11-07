---
id: js/no-invalid-well-known-symbol-methods
title: "No async Symbol.dispose or async Symbol.iterator generator"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "renaming moves the method to the asynchronous protocol: `using` becomes `await using`, `for…of` becomes `for await…of` for its callers"
stability: preview
ported_from: [eslint-plugin-unicorn/no-invalid-well-known-symbol-methods]
since: 0.5.0
---

`Symbol.dispose` and `Symbol.iterator` are synchronous protocols: an `async` method there returns a Promise the protocol does not await, so cleanup runs late or iteration yields Promises. The asynchronous ones are `Symbol.asyncDispose` and `Symbol.asyncIterator`. Upstream also reports a non-generator `Symbol.asyncIterator` and other protocol misuses; those need the body rewritten, and are not reported here.

**Why:** The method looks right and runs, but the protocol it was written for ignores what it returns.

```codelaw
match:
  kind: method_definition
  regex: '^async\b'
  any:
    - all:
        - has: { field: name, kind: computed_property_name, has: { kind: member_expression, regex: '^Symbol\.dispose$', pattern: $D } }
    - all:
        - regex: '^async\s*\*'
        - has: { field: name, kind: computed_property_name, has: { kind: member_expression, regex: '^Symbol\.iterator$', pattern: $I } }
message: "An async method here needs the asynchronous protocol's symbol"
fix:
  - replace: { target: $D, with: Symbol.asyncDispose }
  - replace: { target: $I, with: Symbol.asyncIterator }
```

## Examples

### JavaScript

#### Don't

```js
class Connection {
  async [Symbol.dispose]() {
    await this.close();
  }
}
```

#### Do

```js
class Connection {
  async [Symbol.asyncDispose]() {
    await this.close();
  }
}
```


### JSX

#### Don't

```jsx
class Connection {
  async [Symbol.dispose]() {
    await this.close();
  }
}
```

#### Do

```jsx
class Connection {
  async [Symbol.asyncDispose]() {
    await this.close();
  }
}
```


### TypeScript

#### Don't

```ts
class Connection {
  async [Symbol.dispose]() {
    await this.close();
  }
}
```

#### Do

```ts
class Connection {
  async [Symbol.asyncDispose]() {
    await this.close();
  }
}
```


### TSX

#### Don't

```tsx
class Connection {
  async [Symbol.dispose]() {
    await this.close();
  }
}
```

#### Do

```tsx
class Connection {
  async [Symbol.asyncDispose]() {
    await this.close();
  }
}
```
