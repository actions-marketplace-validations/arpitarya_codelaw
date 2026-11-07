---
id: js/prefer-cypress-should
title: "should() rather than then() around a Cypress assertion"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`should` retries its callback until it passes or times out, so a callback with side effects runs more than once"
stability: preview
ported_from: [eslint-plugin-sonarjs/prefer-cypress-should]
since: 0.5.0
---

`cy.get(…).then($el => { expect(…) })` checks the element once, at the moment it is found. `.should($el => { expect(…) })` retries the check until it passes or the command times out, which is what an assertion on a changing page needs. A `then` callback that only asserts — it holds an `expect` and runs no other Cypress command — is reported.

**Why:** Single-shot checks are the classic source of flaky Cypress tests.

```codelaw
match:
  kind: call_expression
  all:
    - has:
        field: function
        kind: member_expression
        all:
          - has: { field: object, regex: '^cy\.' }
          - has: { field: property, regex: '^then$', pattern: $THEN }
    - has:
        field: arguments
        has:
          nthChild: 1
          any: [{ kind: arrow_function }, { kind: function_expression }]
          all:
            - has: { stopBy: end, kind: call_expression, regex: '^expect\(' }
            - not: { has: { stopBy: end, kind: identifier, regex: '^cy$' } }
            - not: { has: { stopBy: end, kind: return_statement } }
message: "Assert with the retrying `should`"
fix:
  - replace: { target: $THEN, with: should }
```

## Examples

### JavaScript

#### Don't

```js
cy.get("[data-cy=status]").then(($status) => {
  expect($status.text()).to.equal("Ready");
});
```

#### Do

```js
cy.get("[data-cy=status]").should(($status) => {
  expect($status.text()).to.equal("Ready");
});
```


### JSX

#### Don't

```jsx
cy.get("[data-cy=status]").then(($status) => {
  expect($status.text()).to.equal("Ready");
});
```

#### Do

```jsx
cy.get("[data-cy=status]").should(($status) => {
  expect($status.text()).to.equal("Ready");
});
```


### TypeScript

#### Don't

```ts
cy.get("[data-cy=status]").then(($status) => {
  expect($status.text()).to.equal("Ready");
});
```

#### Do

```ts
cy.get("[data-cy=status]").should(($status) => {
  expect($status.text()).to.equal("Ready");
});
```


### TSX

#### Don't

```tsx
cy.get("[data-cy=status]").then(($status) => {
  expect($status.text()).to.equal("Ready");
});
```

#### Do

```tsx
cy.get("[data-cy=status]").should(($status) => {
  expect($status.text()).to.equal("Ready");
});
```
