# react/no-jsx-leaked-dollar — cases

### case: a currency sign that is meant, written as text

```jsx
const t = <p>{"$"}{total}</p>;
```

```expect
(none)
```

### case: a dollar before an expression

```jsx
const t = <p>Pay ${amount} now</p>;
```

```expect
1:19 react/no-jsx-leaked-dollar
```

```fixed
const t = <p>Pay {amount} now</p>;
```
