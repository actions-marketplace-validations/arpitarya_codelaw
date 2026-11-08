# js/misplaced-loop-counter — cases

### case: a list walk

```js
for (let node = head; node.ok; node = node.next) {
  use(node);
}
```

```expect
(none)
```

### case: members

```js
for (; this.a < 3; this.b++) {}
```

```expect
1:1 js/misplaced-loop-counter
```
