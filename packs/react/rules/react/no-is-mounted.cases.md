### case: a guard with an else carries no edit

```jsx
class A extends React.Component {
  done() {
    if (this.isMounted()) {
      this.setState({ ok: true });
    } else {
      log();
    }
  }
}
```

```expect
3:9 react/no-is-mounted
```
