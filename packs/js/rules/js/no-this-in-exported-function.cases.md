# js/no-this-in-exported-function — cases

### case: a method inside the exported function has its own this

```js
export function make() {
  return {
    get() {
      return this.v;
    },
  };
}
```

```expect

```

### case: an arrow inherits this

```js
export default function f() {
  return () => this.x;
}
```

```expect
2:16 js/no-this-in-exported-function
```
