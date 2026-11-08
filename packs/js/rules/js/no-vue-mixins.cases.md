# js/no-vue-mixins — cases

### case: defineComponent

```js
const C = defineComponent({ mixins: [a, b] });
```

```expect
1:29 js/no-vue-mixins
```

### case: an unrelated object stays

```js
const config = { mixins: [a] };
```

```expect
(none)
```
