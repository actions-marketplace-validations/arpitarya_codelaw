---
id: js/prefer-native-lodash-alternative
title: "The built-in rather than a lodash function it duplicates"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "lodash accepts `null`, objects and shorthand predicates where the built-ins throw or differ, so each edit is offered for review"
stability: preview
ported_from: [eslint-plugin-sonarjs/prefer-native-lodash-alternative]
since: 0.5.0
---

Where the language has the same operation, the lodash call is the built-in: `_.isArray(x)` is `Array.isArray(x)`, `_.isNaN(x)` is `Number.isNaN(x)`, `_.keys(o)`, `_.values(o)` and `_.toPairs(o)` are `Object.keys`, `Object.values` and `Object.entries`, and `_.assign(…)` is `Object.assign(…)`. Upstream's catalogue also covers the array helpers (`_.map`, `_.filter`, `_.includes`, …), whose lodash forms take objects and shorthands; those are not reported here.

**Why:** One fewer dependency for operations the platform already has.

```codelaw
match:
  kind: call_expression
  all:
    - has: { field: function, kind: member_expression, all: [{ has: { field: object, regex: '^(_|lodash)$' } }, { has: { field: property, regex: '^(isArray|isNaN|keys|values|toPairs|assign)$', pattern: $FN } }] }
    - has: { field: arguments, pattern: $ARGS }
message: "Use the built-in for `$FN`"
fix: "$FN|map(isArray:Array.isArray,isNaN:Number.isNaN,keys:Object.keys,values:Object.values,toPairs:Object.entries,assign:Object.assign)$ARGS"
```

## Examples

### JavaScript

#### Don't

```js
const names = _.keys(users);
```

#### Do

```js
const names = Object.keys(users);
```


### JSX

#### Don't

```jsx
const names = _.keys(users);
```

#### Do

```jsx
const names = Object.keys(users);
```


### TypeScript

#### Don't

```ts
const names = _.keys(users);
```

#### Do

```ts
const names = Object.keys(users);
```


### TSX

#### Don't

```tsx
const names = _.keys(users);
```

#### Do

```tsx
const names = Object.keys(users);
```
