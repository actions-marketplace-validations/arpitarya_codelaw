# react/self-closing-comp — cases

### case: a character reference is a child

```jsx
const b = <button type="button">&lt;</button>;
```

```expect
```

### case: only a line break between the tags

```jsx
const a = <div className="a">
</div>;
```

```expect
1:11 react/self-closing-comp
```

```fixed
const a = <div className="a" />;
```

### case: only spaces on one line

```jsx
const a = <div>   </div>;
```

```expect
1:11 react/self-closing-comp
```

```fixed
const a = <div />;
```
