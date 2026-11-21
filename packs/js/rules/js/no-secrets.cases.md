# js/no-secrets — cases

### case: a high-entropy token

```js
const token = "wJalrXUtnFEMI7K7MDENGbPxRfiCYzq9";
```

```expect
1:15 js/no-secrets
```

### case: a sentence and an import

```js
import x from "some-package-with-a-long-name-in-it";
const s = "a perfectly ordinary sentence of many words";
```

```expect
(none)
```

### case: a private key block

```js
const k = "-----BEGIN RSA PRIVATE KEY-----";
```

```expect
1:11 js/no-secrets
```
