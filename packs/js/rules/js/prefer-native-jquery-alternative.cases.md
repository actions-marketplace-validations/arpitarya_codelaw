# js/prefer-native-jquery-alternative — cases

### case: trim

```js
const name = jQuery.trim(input);
```

```expect
1:14 js/prefer-native-jquery-alternative
```

```fixed
const name = input.trim();
```

### case: inArray

```js
const i = $.inArray("x", list);
```

```expect
1:11 js/prefer-native-jquery-alternative
```

```fixed
const i = list.indexOf("x");
```
