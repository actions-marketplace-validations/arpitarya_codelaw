---
id: js/require-text-decoder-streaming
title: "decode(chunk, { stream: true }) inside a loop over a fetch body"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "with `stream: true` the decoder holds back an incomplete character until the next chunk, so a final `decode()` call may be needed after the loop"
stability: preview
ported_from: [eslint-plugin-unicorn/require-text-decoder-streaming]
since: 0.5.0
---

A multibyte character can be split between two chunks of a fetch body. Decoding each chunk alone turns its halves into replacement characters; `decoder.decode(chunk, { stream: true })` keeps the partial bytes for the next call. A `decode` of the loop variable inside `for await (const chunk of response.body)` with no options is reported.

**Why:** The text is right for ASCII and wrong for `€`, so tests pass and users see `���`.

```codelaw
match:
  kind: call_expression
  pattern: "$D.decode($C)"
  inside:
    stopBy: end
    kind: for_in_statement
    regex: '^for\s+await'
    all:
      - has: { field: left, pattern: $C }
      - has: { field: right, kind: member_expression, regex: '\.body$' }
message: "Decode each chunk with `{ stream: true }`"
fix: "$D.decode($C, { stream: true })"
```

## Examples

### JavaScript

#### Don't

```js
for await (const chunk of response.body) {
  text += decoder.decode(chunk);
}
```

#### Do

```js
for await (const chunk of response.body) {
  text += decoder.decode(chunk, { stream: true });
}
```


### JSX

#### Don't

```jsx
for await (const chunk of response.body) {
  text += decoder.decode(chunk);
}
```

#### Do

```jsx
for await (const chunk of response.body) {
  text += decoder.decode(chunk, { stream: true });
}
```


### TypeScript

#### Don't

```ts
for await (const chunk of response.body) {
  text += decoder.decode(chunk);
}
```

#### Do

```ts
for await (const chunk of response.body) {
  text += decoder.decode(chunk, { stream: true });
}
```


### TSX

#### Don't

```tsx
for await (const chunk of response.body) {
  text += decoder.decode(chunk);
}
```

#### Do

```tsx
for await (const chunk of response.body) {
  text += decoder.decode(chunk, { stream: true });
}
```
