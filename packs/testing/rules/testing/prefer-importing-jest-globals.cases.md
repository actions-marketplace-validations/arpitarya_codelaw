# testing/prefer-importing-jest-globals — cases

### case: merged into the existing import

```ts
import { jest } from "@jest/globals";

it("a", () => {
  jest.fn();
});
```

```expect
3:1 testing/prefer-importing-jest-globals
```

```fixed
import { it, jest } from '@jest/globals';

it("a", () => {
  jest.fn();
});
```

### case: a CommonJS file requires

```js
const fs = require("fs");

test("a", () => {});
```

```expect
3:1 testing/prefer-importing-jest-globals
```

```fixed
const { test } = require('@jest/globals');
const fs = require("fs");

test("a", () => {});
```

### case: everything imported

```ts
import { describe, expect, it } from "@jest/globals";

describe("a", () => {
  it("b", () => {
    expect(1).toBe(1);
  });
});
```

```expect
(none)
```

### case: a local function named like a global

```ts
function it(name: string) {}

it("not a test");
```

```expect
(none)
```

### case: expect with no matcher is not a Jest call

```ts
expect(1);
```

```expect
(none)
```

### case: the hooks and each

```ts
describe.each([1, 2])("n %i", () => {
  beforeEach(() => {});
});
```

```expect
1:1 testing/prefer-importing-jest-globals
```

```fixed
import { beforeEach, describe } from '@jest/globals';
describe.each([1, 2])("n %i", () => {
  beforeEach(() => {});
});
```
