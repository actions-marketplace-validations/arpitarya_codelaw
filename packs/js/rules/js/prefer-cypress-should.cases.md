# js/prefer-cypress-should — cases

### case: a then that runs a command stays

```js
cy.get("a").then(($a) => {
  cy.visit($a.attr("href"));
});
```

```expect
(none)
```
