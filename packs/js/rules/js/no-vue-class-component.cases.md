# js/no-vue-class-component — cases

### case: a decorated class

```ts
@Component
class Counter {}
```

```expect
1:1 js/no-vue-class-component
```

### case: another base class stays

```js
class Counter extends Base {}
```

```expect
(none)
```
