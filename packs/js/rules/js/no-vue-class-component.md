---
id: js/no-vue-class-component
title: "No Vue class components"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "moving a component to the Composition API rewrites its whole body"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-sonarjs/no-vue-class-component]
since: 0.5.0
---

Class-based Vue components — a class extending `Vue`, or decorated with `@Component` / `@Options` from `vue-class-component` or `vue-property-decorator` — are deprecated: the libraries are unmaintained and Vue 3 is built around the Composition API. Such a class is reported.

**Why:** The pattern locks the code to libraries that no longer follow Vue's releases.

```codelaw
match:
  all:
  - any:
    - kind: class_declaration
    - kind: class
  - any:
    - has: { kind: class_heritage, has: { stopBy: end, kind: identifier, regex: '^Vue$' } }
    - has: { kind: decorator, regex: '^@(Component|Options)\b' }
    - inside: { kind: export_statement, has: { kind: decorator, regex: '^@(Component|Options)\b' } }
message: "Rewrite this class component with the Composition API"
```

## How to fix

1. Rewrite the component with `defineComponent` and `setup()` (or `<script setup>`): data as `ref`/`reactive`, computed getters as `computed`, methods as functions.

## Examples

### JavaScript

#### Don't

```js
export default class Counter extends Vue {
  count = 0;
}
```

#### Do

```js
export default defineComponent({
  setup() {
    const count = ref(0);
    return { count };
  },
});
```


### JSX

#### Don't

```jsx
export default class Counter extends Vue {
  count = 0;
}
```

#### Do

```jsx
export default defineComponent({
  setup() {
    const count = ref(0);
    return { count };
  },
});
```


### TypeScript

#### Don't

```ts
export default class Counter extends Vue {
  count = 0;
}
```

#### Do

```ts
export default defineComponent({
  setup() {
    const count = ref(0);
    return { count };
  },
});
```


### TSX

#### Don't

```tsx
export default class Counter extends Vue {
  count = 0;
}
```

#### Do

```tsx
export default defineComponent({
  setup() {
    const count = ref(0);
    return { count };
  },
});
```
