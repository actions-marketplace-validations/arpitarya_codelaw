# js/for-direction — cases

### case: a prefix decrement is flipped

```js
for (let i = 0; i < 10; --i) draw(i);
```

```expect
1:1 js/for-direction
```

```fixed
for (let i = 0; i < 10; ++i) draw(i);
```
