# js/stable-tests — cases

### case: jest

```js
jest.retryTimes(2);
```

```expect
1:1 js/stable-tests
```

### case: zero stays

```js
jest.retryTimes(0);
```

```expect
(none)
```
