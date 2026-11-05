# react/a11y-alt-text — cases

### case: a spread may carry the alt

```jsx
const a = <img {...props} />;
```

```expect

```

### case: an input of type image

```jsx
const a = <input type="image" src="go.png" />;
```

```expect
1:11 react/a11y-alt-text
```

### case: an img hidden from assistive technology gets an empty alt

```jsx
const a = <img src="deco.png" aria-hidden="true" />;
```

```expect
1:11 react/a11y-alt-text
```

```fixed
const a = <img alt="" src="deco.png" aria-hidden="true" />;
```
