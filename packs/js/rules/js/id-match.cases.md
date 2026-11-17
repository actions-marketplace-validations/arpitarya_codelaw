# js/id-match — cases

### case: silent without the option

```js
const user_name = load();
```

```expect
(none)
```

### case: a declaration, a parameter and a function

```js options={"pattern":"^[a-z][a-zA-Z0-9]*$"}
const user_name = 1;
function get_user(user_id) {
  return user_id;
}
```

```expect
1:7 js/id-match
2:10 js/id-match
2:19 js/id-match
3:10 js/id-match
```

### case: a call's arguments are not checked

```js options={"pattern":"^[a-z][a-zA-Z0-9]*$"}
const total = sum(user_name);
```

```expect
(none)
```

### case: properties are checked only with the option

```js options={"pattern":"^[a-z][a-zA-Z0-9]*$"}
const o = { my_key: 1 };
o.other_key = 2;
```

```expect
(none)
```

### case: properties

```js options={"pattern":"^[a-z][a-zA-Z0-9]*$","properties":true}
const o = { my_key: 1 };
o.other_key = 2;
```

```expect
1:13 js/id-match
2:3 js/id-match
```

### case: only declarations

```js options={"pattern":"^[a-z][a-zA-Z0-9]*$","onlyDeclarations":true}
const user_name = 1;
const copy = user_name + 1;
```

```expect
1:7 js/id-match
```

### case: destructuring

```js options={"pattern":"^[a-z][a-zA-Z0-9]*$","ignoreDestructuring":true}
const { user_name } = source;
const { id: user_id } = source;
```

```expect
2:13 js/id-match
```

### case: class fields

```js options={"pattern":"^[a-z][a-zA-Z0-9]*$","classFields":true}
class Store {
  my_field = 1;
  #my_private = 2;
}
```

```expect
2:3 js/id-match
3:3 js/id-match
```

### case: a private field is read only with classFields, a private method always

```js options={"pattern":"^[a-z][a-zA-Z0-9]*$"}
class Store {
  my_field = 1;
  #my_method() {}
}
```

```expect
3:3 js/id-match
```

### case: a pattern that is not a regular expression

```js options={"pattern":"("}
const a = 1;
```

```expect
1:7 js/id-match
```
