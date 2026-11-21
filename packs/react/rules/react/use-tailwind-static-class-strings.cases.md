# react/use-tailwind-static-class-strings — cases

### case: a string joined with plus

```jsx
const d = <div className={"p-4 " + extra} />;
```

```expect
1:16 react/use-tailwind-static-class-strings
```

### case: a conditional of complete names

```jsx
const d = <div className={on ? "bg-red-500" : "bg-blue-500"} />;
```

```expect
(none)
```

### case: a template without a substitution

```jsx
const d = <div className={`p-4`} />;
```

```expect
(none)
```
