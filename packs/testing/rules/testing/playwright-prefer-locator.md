---
id: testing/playwright-prefer-locator
title: "Locator methods over `page` actions"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: suggest
fix_reason: "a locator is strict — it fails on more than one match where the page method took the first"
stability: preview
ported_from: ["eslint-plugin-playwright/prefer-locator"]
since: 0.5.0
---

An action or query on a selector is made through a locator: `page.locator(sel).click()`, not `page.click(sel)` (and likewise `fill`, `check`, `hover`, `textContent`, `isVisible` and the rest). The suggested edit makes the change.

**Why:** The page-level selector methods are discouraged: a locator is strict, re-finds the element and composes with filters.

```codelaw
utils:
  expect-call:
    kind: call_expression
    has:
      field: function
      any:
        - { kind: identifier, regex: '^expect$' }
        - kind: member_expression
          all:
            - has: { field: property, regex: '^(soft|poll)$' }
            - has: { field: object, kind: identifier, regex: '^expect$' }
  expect-chain:
    any:
      - matches: expect-call
      - kind: member_expression
        has:
          field: object
          any:
            - matches: expect-call
            - { kind: member_expression, has: { field: object, matches: expect-call } }
  matcher-call:
    kind: call_expression
    has:
      field: function
      kind: member_expression
      has: { field: object, matches: expect-chain }
  test-name:
    any:
      - { kind: identifier, regex: '^(it|test|xit|xtest|fit)$' }
      - kind: member_expression
        all:
          - has: { field: property, regex: '^(only|skip|concurrent|failing|todo|fixme|fail|slow)$' }
          - regex: '^(it|test|xit|xtest|fit)(\.(only|skip|concurrent|failing|todo|fixme|fail|slow))+$'
  describe-name:
    any:
      - { kind: identifier, regex: '^(describe|xdescribe|fdescribe|suite)$' }
      - kind: member_expression
        all:
          - has: { field: property, regex: '^(describe|only|skip|serial|parallel|concurrent|fixme)$' }
          - regex: '^(describe|xdescribe|fdescribe|test\.describe|suite)(\.(only|skip|serial|parallel|concurrent|fixme))*$'
  test-call:
    kind: call_expression
    has:
      field: function
      any:
        - matches: test-name
        - kind: call_expression
          has:
            field: function
            kind: member_expression
            all:
              - has: { field: property, regex: '^each$' }
              - regex: '^(it|test|xit|xtest|fit)(\.(only|skip|concurrent|failing))*\.each$'
  describe-call:
    kind: call_expression
    has:
      field: function
      any:
        - matches: describe-name
        - kind: call_expression
          has:
            field: function
            kind: member_expression
            all:
              - has: { field: property, regex: '^each$' }
              - regex: '^(describe|xdescribe|fdescribe|suite)(\.(only|skip|concurrent))*\.each$'
  hook-call:
    kind: call_expression
    has:
      field: function
      any:
        - { kind: identifier, regex: '^(beforeAll|beforeEach|afterAll|afterEach)$' }
        - kind: member_expression
          all:
            - has: { field: property, regex: '^(beforeAll|beforeEach|afterAll|afterEach)$' }
            - has: { field: object, kind: identifier, regex: '^test$' }
  test-fn:
    any: [{ kind: arrow_function }, { kind: function_expression }]
    inside: { kind: arguments, inside: { matches: test-call } }
  describe-fn:
    any: [{ kind: arrow_function }, { kind: function_expression }]
    inside: { kind: arguments, inside: { matches: describe-call } }
  in-test:
    inside: { stopBy: end, any: [{ matches: test-call }, { matches: hook-call }] }
  block-level:
    kind: expression_statement
    any:
      - inside: { kind: program }
      - inside: { kind: statement_block, inside: { matches: describe-fn } }
match:
  kind: call_expression
  any:
    - all: [{ pattern: "page.click($S)" }, { pattern: $W0 }]
    - all: [{ pattern: "page.dblclick($S)" }, { pattern: $W1 }]
    - all: [{ pattern: "page.fill($S)" }, { pattern: $W2 }]
    - all: [{ pattern: "page.type($S)" }, { pattern: $W3 }]
    - all: [{ pattern: "page.press($S)" }, { pattern: $W4 }]
    - all: [{ pattern: "page.pressSequentially($S)" }, { pattern: $W5 }]
    - all: [{ pattern: "page.check($S)" }, { pattern: $W6 }]
    - all: [{ pattern: "page.uncheck($S)" }, { pattern: $W7 }]
    - all: [{ pattern: "page.setChecked($S)" }, { pattern: $W8 }]
    - all: [{ pattern: "page.hover($S)" }, { pattern: $W9 }]
    - all: [{ pattern: "page.focus($S)" }, { pattern: $W10 }]
    - all: [{ pattern: "page.tap($S)" }, { pattern: $W11 }]
    - all: [{ pattern: "page.selectOption($S)" }, { pattern: $W12 }]
    - all: [{ pattern: "page.setInputFiles($S)" }, { pattern: $W13 }]
    - all: [{ pattern: "page.dispatchEvent($S)" }, { pattern: $W14 }]
    - all: [{ pattern: "page.textContent($S)" }, { pattern: $W15 }]
    - all: [{ pattern: "page.innerText($S)" }, { pattern: $W16 }]
    - all: [{ pattern: "page.innerHTML($S)" }, { pattern: $W17 }]
    - all: [{ pattern: "page.getAttribute($S)" }, { pattern: $W18 }]
    - all: [{ pattern: "page.inputValue($S)" }, { pattern: $W19 }]
    - all: [{ pattern: "page.isChecked($S)" }, { pattern: $W20 }]
    - all: [{ pattern: "page.isDisabled($S)" }, { pattern: $W21 }]
    - all: [{ pattern: "page.isEditable($S)" }, { pattern: $W22 }]
    - all: [{ pattern: "page.isEnabled($S)" }, { pattern: $W23 }]
    - all: [{ pattern: "page.isHidden($S)" }, { pattern: $W24 }]
    - all: [{ pattern: "page.isVisible($S)" }, { pattern: $W25 }]
    - all: [{ pattern: "frame.click($S)" }, { pattern: $W26 }]
    - all: [{ pattern: "frame.dblclick($S)" }, { pattern: $W27 }]
    - all: [{ pattern: "frame.fill($S)" }, { pattern: $W28 }]
    - all: [{ pattern: "frame.type($S)" }, { pattern: $W29 }]
    - all: [{ pattern: "frame.press($S)" }, { pattern: $W30 }]
    - all: [{ pattern: "frame.pressSequentially($S)" }, { pattern: $W31 }]
    - all: [{ pattern: "frame.check($S)" }, { pattern: $W32 }]
    - all: [{ pattern: "frame.uncheck($S)" }, { pattern: $W33 }]
    - all: [{ pattern: "frame.setChecked($S)" }, { pattern: $W34 }]
    - all: [{ pattern: "frame.hover($S)" }, { pattern: $W35 }]
    - all: [{ pattern: "frame.focus($S)" }, { pattern: $W36 }]
    - all: [{ pattern: "frame.tap($S)" }, { pattern: $W37 }]
    - all: [{ pattern: "frame.selectOption($S)" }, { pattern: $W38 }]
    - all: [{ pattern: "frame.setInputFiles($S)" }, { pattern: $W39 }]
    - all: [{ pattern: "frame.dispatchEvent($S)" }, { pattern: $W40 }]
    - all: [{ pattern: "frame.textContent($S)" }, { pattern: $W41 }]
    - all: [{ pattern: "frame.innerText($S)" }, { pattern: $W42 }]
    - all: [{ pattern: "frame.innerHTML($S)" }, { pattern: $W43 }]
    - all: [{ pattern: "frame.getAttribute($S)" }, { pattern: $W44 }]
    - all: [{ pattern: "frame.inputValue($S)" }, { pattern: $W45 }]
    - all: [{ pattern: "frame.isChecked($S)" }, { pattern: $W46 }]
    - all: [{ pattern: "frame.isDisabled($S)" }, { pattern: $W47 }]
    - all: [{ pattern: "frame.isEditable($S)" }, { pattern: $W48 }]
    - all: [{ pattern: "frame.isEnabled($S)" }, { pattern: $W49 }]
    - all: [{ pattern: "frame.isHidden($S)" }, { pattern: $W50 }]
    - all: [{ pattern: "frame.isVisible($S)" }, { pattern: $W51 }]
    - all: [{ pattern: "page.click($S, $$$R)" }, { pattern: $W52 }]
    - all: [{ pattern: "page.dblclick($S, $$$R)" }, { pattern: $W53 }]
    - all: [{ pattern: "page.fill($S, $$$R)" }, { pattern: $W54 }]
    - all: [{ pattern: "page.type($S, $$$R)" }, { pattern: $W55 }]
    - all: [{ pattern: "page.press($S, $$$R)" }, { pattern: $W56 }]
    - all: [{ pattern: "page.pressSequentially($S, $$$R)" }, { pattern: $W57 }]
    - all: [{ pattern: "page.check($S, $$$R)" }, { pattern: $W58 }]
    - all: [{ pattern: "page.uncheck($S, $$$R)" }, { pattern: $W59 }]
    - all: [{ pattern: "page.setChecked($S, $$$R)" }, { pattern: $W60 }]
    - all: [{ pattern: "page.hover($S, $$$R)" }, { pattern: $W61 }]
    - all: [{ pattern: "page.focus($S, $$$R)" }, { pattern: $W62 }]
    - all: [{ pattern: "page.tap($S, $$$R)" }, { pattern: $W63 }]
    - all: [{ pattern: "page.selectOption($S, $$$R)" }, { pattern: $W64 }]
    - all: [{ pattern: "page.setInputFiles($S, $$$R)" }, { pattern: $W65 }]
    - all: [{ pattern: "page.dispatchEvent($S, $$$R)" }, { pattern: $W66 }]
    - all: [{ pattern: "page.textContent($S, $$$R)" }, { pattern: $W67 }]
    - all: [{ pattern: "page.innerText($S, $$$R)" }, { pattern: $W68 }]
    - all: [{ pattern: "page.innerHTML($S, $$$R)" }, { pattern: $W69 }]
    - all: [{ pattern: "page.getAttribute($S, $$$R)" }, { pattern: $W70 }]
    - all: [{ pattern: "page.inputValue($S, $$$R)" }, { pattern: $W71 }]
    - all: [{ pattern: "page.isChecked($S, $$$R)" }, { pattern: $W72 }]
    - all: [{ pattern: "page.isDisabled($S, $$$R)" }, { pattern: $W73 }]
    - all: [{ pattern: "page.isEditable($S, $$$R)" }, { pattern: $W74 }]
    - all: [{ pattern: "page.isEnabled($S, $$$R)" }, { pattern: $W75 }]
    - all: [{ pattern: "page.isHidden($S, $$$R)" }, { pattern: $W76 }]
    - all: [{ pattern: "page.isVisible($S, $$$R)" }, { pattern: $W77 }]
    - all: [{ pattern: "frame.click($S, $$$R)" }, { pattern: $W78 }]
    - all: [{ pattern: "frame.dblclick($S, $$$R)" }, { pattern: $W79 }]
    - all: [{ pattern: "frame.fill($S, $$$R)" }, { pattern: $W80 }]
    - all: [{ pattern: "frame.type($S, $$$R)" }, { pattern: $W81 }]
    - all: [{ pattern: "frame.press($S, $$$R)" }, { pattern: $W82 }]
    - all: [{ pattern: "frame.pressSequentially($S, $$$R)" }, { pattern: $W83 }]
    - all: [{ pattern: "frame.check($S, $$$R)" }, { pattern: $W84 }]
    - all: [{ pattern: "frame.uncheck($S, $$$R)" }, { pattern: $W85 }]
    - all: [{ pattern: "frame.setChecked($S, $$$R)" }, { pattern: $W86 }]
    - all: [{ pattern: "frame.hover($S, $$$R)" }, { pattern: $W87 }]
    - all: [{ pattern: "frame.focus($S, $$$R)" }, { pattern: $W88 }]
    - all: [{ pattern: "frame.tap($S, $$$R)" }, { pattern: $W89 }]
    - all: [{ pattern: "frame.selectOption($S, $$$R)" }, { pattern: $W90 }]
    - all: [{ pattern: "frame.setInputFiles($S, $$$R)" }, { pattern: $W91 }]
    - all: [{ pattern: "frame.dispatchEvent($S, $$$R)" }, { pattern: $W92 }]
    - all: [{ pattern: "frame.textContent($S, $$$R)" }, { pattern: $W93 }]
    - all: [{ pattern: "frame.innerText($S, $$$R)" }, { pattern: $W94 }]
    - all: [{ pattern: "frame.innerHTML($S, $$$R)" }, { pattern: $W95 }]
    - all: [{ pattern: "frame.getAttribute($S, $$$R)" }, { pattern: $W96 }]
    - all: [{ pattern: "frame.inputValue($S, $$$R)" }, { pattern: $W97 }]
    - all: [{ pattern: "frame.isChecked($S, $$$R)" }, { pattern: $W98 }]
    - all: [{ pattern: "frame.isDisabled($S, $$$R)" }, { pattern: $W99 }]
    - all: [{ pattern: "frame.isEditable($S, $$$R)" }, { pattern: $W100 }]
    - all: [{ pattern: "frame.isEnabled($S, $$$R)" }, { pattern: $W101 }]
    - all: [{ pattern: "frame.isHidden($S, $$$R)" }, { pattern: $W102 }]
    - all: [{ pattern: "frame.isVisible($S, $$$R)" }, { pattern: $W103 }]
constraints:
  S: { any: [{ kind: string }, { kind: template_string }] }
message: "Use a locator for this action"
fix:
  - replace: { target: $W0, with: 'page.locator($S).click()' }
  - replace: { target: $W1, with: 'page.locator($S).dblclick()' }
  - replace: { target: $W2, with: 'page.locator($S).fill()' }
  - replace: { target: $W3, with: 'page.locator($S).type()' }
  - replace: { target: $W4, with: 'page.locator($S).press()' }
  - replace: { target: $W5, with: 'page.locator($S).pressSequentially()' }
  - replace: { target: $W6, with: 'page.locator($S).check()' }
  - replace: { target: $W7, with: 'page.locator($S).uncheck()' }
  - replace: { target: $W8, with: 'page.locator($S).setChecked()' }
  - replace: { target: $W9, with: 'page.locator($S).hover()' }
  - replace: { target: $W10, with: 'page.locator($S).focus()' }
  - replace: { target: $W11, with: 'page.locator($S).tap()' }
  - replace: { target: $W12, with: 'page.locator($S).selectOption()' }
  - replace: { target: $W13, with: 'page.locator($S).setInputFiles()' }
  - replace: { target: $W14, with: 'page.locator($S).dispatchEvent()' }
  - replace: { target: $W15, with: 'page.locator($S).textContent()' }
  - replace: { target: $W16, with: 'page.locator($S).innerText()' }
  - replace: { target: $W17, with: 'page.locator($S).innerHTML()' }
  - replace: { target: $W18, with: 'page.locator($S).getAttribute()' }
  - replace: { target: $W19, with: 'page.locator($S).inputValue()' }
  - replace: { target: $W20, with: 'page.locator($S).isChecked()' }
  - replace: { target: $W21, with: 'page.locator($S).isDisabled()' }
  - replace: { target: $W22, with: 'page.locator($S).isEditable()' }
  - replace: { target: $W23, with: 'page.locator($S).isEnabled()' }
  - replace: { target: $W24, with: 'page.locator($S).isHidden()' }
  - replace: { target: $W25, with: 'page.locator($S).isVisible()' }
  - replace: { target: $W26, with: 'frame.locator($S).click()' }
  - replace: { target: $W27, with: 'frame.locator($S).dblclick()' }
  - replace: { target: $W28, with: 'frame.locator($S).fill()' }
  - replace: { target: $W29, with: 'frame.locator($S).type()' }
  - replace: { target: $W30, with: 'frame.locator($S).press()' }
  - replace: { target: $W31, with: 'frame.locator($S).pressSequentially()' }
  - replace: { target: $W32, with: 'frame.locator($S).check()' }
  - replace: { target: $W33, with: 'frame.locator($S).uncheck()' }
  - replace: { target: $W34, with: 'frame.locator($S).setChecked()' }
  - replace: { target: $W35, with: 'frame.locator($S).hover()' }
  - replace: { target: $W36, with: 'frame.locator($S).focus()' }
  - replace: { target: $W37, with: 'frame.locator($S).tap()' }
  - replace: { target: $W38, with: 'frame.locator($S).selectOption()' }
  - replace: { target: $W39, with: 'frame.locator($S).setInputFiles()' }
  - replace: { target: $W40, with: 'frame.locator($S).dispatchEvent()' }
  - replace: { target: $W41, with: 'frame.locator($S).textContent()' }
  - replace: { target: $W42, with: 'frame.locator($S).innerText()' }
  - replace: { target: $W43, with: 'frame.locator($S).innerHTML()' }
  - replace: { target: $W44, with: 'frame.locator($S).getAttribute()' }
  - replace: { target: $W45, with: 'frame.locator($S).inputValue()' }
  - replace: { target: $W46, with: 'frame.locator($S).isChecked()' }
  - replace: { target: $W47, with: 'frame.locator($S).isDisabled()' }
  - replace: { target: $W48, with: 'frame.locator($S).isEditable()' }
  - replace: { target: $W49, with: 'frame.locator($S).isEnabled()' }
  - replace: { target: $W50, with: 'frame.locator($S).isHidden()' }
  - replace: { target: $W51, with: 'frame.locator($S).isVisible()' }
  - replace: { target: $W52, with: 'page.locator($S).click($$$R)' }
  - replace: { target: $W53, with: 'page.locator($S).dblclick($$$R)' }
  - replace: { target: $W54, with: 'page.locator($S).fill($$$R)' }
  - replace: { target: $W55, with: 'page.locator($S).type($$$R)' }
  - replace: { target: $W56, with: 'page.locator($S).press($$$R)' }
  - replace: { target: $W57, with: 'page.locator($S).pressSequentially($$$R)' }
  - replace: { target: $W58, with: 'page.locator($S).check($$$R)' }
  - replace: { target: $W59, with: 'page.locator($S).uncheck($$$R)' }
  - replace: { target: $W60, with: 'page.locator($S).setChecked($$$R)' }
  - replace: { target: $W61, with: 'page.locator($S).hover($$$R)' }
  - replace: { target: $W62, with: 'page.locator($S).focus($$$R)' }
  - replace: { target: $W63, with: 'page.locator($S).tap($$$R)' }
  - replace: { target: $W64, with: 'page.locator($S).selectOption($$$R)' }
  - replace: { target: $W65, with: 'page.locator($S).setInputFiles($$$R)' }
  - replace: { target: $W66, with: 'page.locator($S).dispatchEvent($$$R)' }
  - replace: { target: $W67, with: 'page.locator($S).textContent($$$R)' }
  - replace: { target: $W68, with: 'page.locator($S).innerText($$$R)' }
  - replace: { target: $W69, with: 'page.locator($S).innerHTML($$$R)' }
  - replace: { target: $W70, with: 'page.locator($S).getAttribute($$$R)' }
  - replace: { target: $W71, with: 'page.locator($S).inputValue($$$R)' }
  - replace: { target: $W72, with: 'page.locator($S).isChecked($$$R)' }
  - replace: { target: $W73, with: 'page.locator($S).isDisabled($$$R)' }
  - replace: { target: $W74, with: 'page.locator($S).isEditable($$$R)' }
  - replace: { target: $W75, with: 'page.locator($S).isEnabled($$$R)' }
  - replace: { target: $W76, with: 'page.locator($S).isHidden($$$R)' }
  - replace: { target: $W77, with: 'page.locator($S).isVisible($$$R)' }
  - replace: { target: $W78, with: 'frame.locator($S).click($$$R)' }
  - replace: { target: $W79, with: 'frame.locator($S).dblclick($$$R)' }
  - replace: { target: $W80, with: 'frame.locator($S).fill($$$R)' }
  - replace: { target: $W81, with: 'frame.locator($S).type($$$R)' }
  - replace: { target: $W82, with: 'frame.locator($S).press($$$R)' }
  - replace: { target: $W83, with: 'frame.locator($S).pressSequentially($$$R)' }
  - replace: { target: $W84, with: 'frame.locator($S).check($$$R)' }
  - replace: { target: $W85, with: 'frame.locator($S).uncheck($$$R)' }
  - replace: { target: $W86, with: 'frame.locator($S).setChecked($$$R)' }
  - replace: { target: $W87, with: 'frame.locator($S).hover($$$R)' }
  - replace: { target: $W88, with: 'frame.locator($S).focus($$$R)' }
  - replace: { target: $W89, with: 'frame.locator($S).tap($$$R)' }
  - replace: { target: $W90, with: 'frame.locator($S).selectOption($$$R)' }
  - replace: { target: $W91, with: 'frame.locator($S).setInputFiles($$$R)' }
  - replace: { target: $W92, with: 'frame.locator($S).dispatchEvent($$$R)' }
  - replace: { target: $W93, with: 'frame.locator($S).textContent($$$R)' }
  - replace: { target: $W94, with: 'frame.locator($S).innerText($$$R)' }
  - replace: { target: $W95, with: 'frame.locator($S).innerHTML($$$R)' }
  - replace: { target: $W96, with: 'frame.locator($S).getAttribute($$$R)' }
  - replace: { target: $W97, with: 'frame.locator($S).inputValue($$$R)' }
  - replace: { target: $W98, with: 'frame.locator($S).isChecked($$$R)' }
  - replace: { target: $W99, with: 'frame.locator($S).isDisabled($$$R)' }
  - replace: { target: $W100, with: 'frame.locator($S).isEditable($$$R)' }
  - replace: { target: $W101, with: 'frame.locator($S).isEnabled($$$R)' }
  - replace: { target: $W102, with: 'frame.locator($S).isHidden($$$R)' }
  - replace: { target: $W103, with: 'frame.locator($S).isVisible($$$R)' }
```

## Examples

### JavaScript

#### Don't

```js
test("t", async ({ page }) => {
  await page.click("#save");
  await page.fill("#name", "Ann");
});
```

#### Do

```js
test("t", async ({ page }) => {
  await page.locator("#save").click();
  await page.locator("#name").fill("Ann");
});
```

### JSX

#### Don't

```jsx
test("t", async ({ page }) => {
  await page.click("#save");
  await page.fill("#name", "Ann");
});
```

#### Do

```jsx
test("t", async ({ page }) => {
  await page.locator("#save").click();
  await page.locator("#name").fill("Ann");
});
```

### TypeScript

#### Don't

```ts
test("t", async ({ page }) => {
  await page.click("#save");
  await page.fill("#name", "Ann");
});
```

#### Do

```ts
test("t", async ({ page }) => {
  await page.locator("#save").click();
  await page.locator("#name").fill("Ann");
});
```

### TSX

#### Don't

```tsx
test("t", async ({ page }) => {
  await page.click("#save");
  await page.fill("#name", "Ann");
});
```

#### Do

```tsx
test("t", async ({ page }) => {
  await page.locator("#save").click();
  await page.locator("#name").fill("Ann");
});
```
