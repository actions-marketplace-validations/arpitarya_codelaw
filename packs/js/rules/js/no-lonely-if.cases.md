# js/no-lonely-if — cases

### case: an else with more than the if is left alone

```js
if (a) {
} else {
  if (b) {}
  c();
}
```

```expect

```

### case: reports the else

```js
if (a) {} else { if (b) {} }
```

```expect
1:11 js/no-lonely-if
```
