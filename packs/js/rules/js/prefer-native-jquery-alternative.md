---
id: js/prefer-native-jquery-alternative
title: "The built-in rather than a jQuery utility"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the jQuery versions accept `null` and array-likes the built-ins treat differently (`$.trim(null)` is `\"\"`), so each edit is offered for review"
stability: preview
ported_from: [eslint-plugin-sonarjs/prefer-native-jquery-alternative]
since: 0.5.0
---

`$.isArray(x)` is `Array.isArray(x)`, `$.parseJSON(s)` is `JSON.parse(s)`, `$.now()` is `Date.now()`, `$.trim(s)` is `s.trim()` and `$.inArray(v, list)` is `list.indexOf(v)`. The first four are deprecated in jQuery 3. `jQuery.` is matched as well as `$.`.

**Why:** The built-ins need no library and say the same thing.

```codelaw
match:
  kind: call_expression
  any:
    - all:
        - any: [{ pattern: "$.isArray($X)" }, { pattern: "jQuery.isArray($X)" }]
        - pattern: $W1
    - all:
        - any: [{ pattern: "$.parseJSON($X)" }, { pattern: "jQuery.parseJSON($X)" }]
        - pattern: $W2
    - all:
        - any: [{ pattern: "$.now()" }, { pattern: "jQuery.now()" }]
        - pattern: $W3
    - all:
        - any: [{ pattern: "$.trim($X)" }, { pattern: "jQuery.trim($X)" }]
        - pattern: $W4
    - all:
        - any: [{ pattern: "$.inArray($X, $L)" }, { pattern: "jQuery.inArray($X, $L)" }]
        - pattern: $W5
message: "Use the built-in instead of jQuery"
fix:
  - replace: { target: $W1, with: "Array.isArray($X)" }
  - replace: { target: $W2, with: "JSON.parse($X)" }
  - replace: { target: $W3, with: "Date.now()" }
  - replace: { target: $W4, with: "$X.trim()" }
  - replace: { target: $W5, with: "$L.indexOf($X)" }
```

## Examples

### JavaScript

#### Don't

```js
if ($.isArray(values)) {}
```

#### Do

```js
if (Array.isArray(values)) {}
```


### JSX

#### Don't

```jsx
if ($.isArray(values)) {}
```

#### Do

```jsx
if (Array.isArray(values)) {}
```


### TypeScript

#### Don't

```ts
if ($.isArray(values)) {}
```

#### Do

```ts
if (Array.isArray(values)) {}
```


### TSX

#### Don't

```tsx
if ($.isArray(values)) {}
```

#### Do

```tsx
if (Array.isArray(values)) {}
```
