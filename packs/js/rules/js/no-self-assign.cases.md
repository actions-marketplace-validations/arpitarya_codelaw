# js/no-self-assign — cases

### case: a property carries no edit

```js
obj.x = obj.x;
```

```expect
1:1 js/no-self-assign
```
