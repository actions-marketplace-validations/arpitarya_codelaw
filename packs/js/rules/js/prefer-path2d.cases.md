# js/prefer-path2d — cases

### case: a loop

```js
for (const p of points) {
  ctx.moveTo(0, 0);
  ctx.lineTo(p.x, p.y);
  ctx.stroke();
}
```

```expect
4:3 js/prefer-path2d
```

### case: a one-off drawing stays

```js
ctx.moveTo(0, 0);
ctx.lineTo(1, 1);
ctx.stroke();
```

```expect
(none)
```
