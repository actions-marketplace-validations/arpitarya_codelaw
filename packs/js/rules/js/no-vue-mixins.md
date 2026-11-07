---
id: js/no-vue-mixins
title: "No Vue mixins"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "turning a mixin into a composable moves its state and methods into a function, which is a rewrite"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-sonarjs/no-vue-mixins]
since: 0.5.0
---

A Vue component's `mixins: [...]` option merges other objects' data and methods into it, with no record of where each name came from and silent clashes. Composables (functions returning refs and functions) make the same reuse explicit. The `mixins` property of an object passed to `defineComponent`, `Vue.extend`, `createApp` or exported as a component is reported.

**Why:** A name used in the template may come from any mixin, and two mixins can overwrite each other.

```codelaw
match:
  kind: pair
  all:
    - has: { field: key, regex: '^["'']?mixins["'']?$' }
    - has: { field: value, kind: array }
  inside:
    kind: object
    any:
      - inside: { kind: arguments, inside: { kind: call_expression, has: { field: function, regex: '^(defineComponent|Vue\.extend|Vue\.component|createApp)$' } } }
      - inside: { kind: export_statement }
message: "Replace this mixin with a composable"
```

## How to fix

1. Move the mixin's state and methods into a `useX()` function that returns them, and call it from `setup()`.

## Examples

### JavaScript

#### Don't

```js
export default {
  mixins: [loggerMixin],
};
```

#### Do

```js
export default {
  setup() {
    const { log } = useLogger();
    return { log };
  },
};
```


### JSX

#### Don't

```jsx
export default {
  mixins: [loggerMixin],
};
```

#### Do

```jsx
export default {
  setup() {
    const { log } = useLogger();
    return { log };
  },
};
```


### TypeScript

#### Don't

```ts
export default {
  mixins: [loggerMixin],
};
```

#### Do

```ts
export default {
  setup() {
    const { log } = useLogger();
    return { log };
  },
};
```


### TSX

#### Don't

```tsx
export default {
  mixins: [loggerMixin],
};
```

#### Do

```tsx
export default {
  setup() {
    const { log } = useLogger();
    return { log };
  },
};
```
