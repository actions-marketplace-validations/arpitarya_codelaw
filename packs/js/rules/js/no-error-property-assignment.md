---
id: js/no-error-property-assignment
title: "No overwriting name, stack, cause or errors on a new built-in error"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the replacement depends on what the value was for: a `cause` option, an error subclass, or a new property"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-unicorn/no-error-property-assignment]
since: 0.5.0
---

A built-in error's `name`, `stack`, `cause` and (on `AggregateError`) `errors` are set by its constructor. Overwriting them right after — `Object.assign(new Error(m), { name })`, or `error.stack = …` on a `const error = new Error(…)` declared just before — makes logs and handlers that read them lie. Unknown values and custom error classes are left.

**Why:** Code that reads `error.name` or `error.stack` trusts the constructor's values.

```codelaw
utils:
  builtin_error:
    kind: new_expression
    has: { field: constructor, regex: '^(Error|TypeError|RangeError|SyntaxError|ReferenceError|EvalError|URIError|AggregateError)$' }
  meta_key:
    kind: pair
    has: { field: key, regex: '^["'']?(name|stack|cause|errors)["'']?$' }
match:
  any:
    - all:
        - pattern: "Object.assign($E, $O)"
        - has: { field: arguments, has: { nthChild: 1, matches: builtin_error } }
        - has: { field: arguments, has: { nthChild: 2, kind: object, has: { any: [{ matches: meta_key }, { kind: shorthand_property_identifier, regex: '^(name|stack|cause|errors)$' }] } } }
    - all:
        - kind: expression_statement
        - has: { kind: assignment_expression, has: { field: left, kind: member_expression, pattern: "$X.$P", has: { field: property, regex: '^(name|stack|cause|errors)$' } } }
        - follows:
            kind: lexical_declaration
            has: { kind: variable_declarator, all: [{ has: { field: name, pattern: $X } }, { has: { field: value, matches: builtin_error } }] }
message: "Don't overwrite the error's own metadata"
```

## How to fix

1. For a cause, pass it to the constructor: `new Error(message, { cause })`.
2. For a different `name`, declare an error subclass that sets it.
3. For extra data, use a property of your own (`code`, `details`).

## Examples

### JavaScript

#### Don't

```js
throw Object.assign(new Error("failed"), { name: "HttpError" });
```

#### Do

```js
throw Object.assign(new Error("failed"), { code: "HTTP_500" });
```


### JSX

#### Don't

```jsx
throw Object.assign(new Error("failed"), { name: "HttpError" });
```

#### Do

```jsx
throw Object.assign(new Error("failed"), { code: "HTTP_500" });
```


### TypeScript

#### Don't

```ts
throw Object.assign(new Error("failed"), { name: "HttpError" });
```

#### Do

```ts
throw Object.assign(new Error("failed"), { code: "HTTP_500" });
```


### TSX

#### Don't

```tsx
throw Object.assign(new Error("failed"), { name: "HttpError" });
```

#### Do

```tsx
throw Object.assign(new Error("failed"), { code: "HTTP_500" });
```
