# js/prefer-native-lodash-alternative — cases

### case: toPairs

```js
const pairs = lodash.toPairs(config);
```

```expect
1:15 js/prefer-native-lodash-alternative
```

```fixed
const pairs = Object.entries(config);
```

### case: map stays

```js
const xs = _.map(list, f);
```

```expect
(none)
```
