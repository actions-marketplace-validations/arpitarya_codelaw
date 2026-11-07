# js/review-blockchain-mnemonic — cases

### case: a template

```js
const a = mnemonicToAccount(`test test test test test test test test test test test junk`);
```

```expect
1:11 js/review-blockchain-mnemonic
```

### case: a short string stays

```js
const a = fromPhrase("not a phrase");
```

```expect
(none)
```
