# testing/prefer-importing-vitest-globals — cases

### case: added to the existing import

```ts
import { describe } from "vitest";

describe("a", () => {
  it("b", () => {});
});
```

```expect
4:3 testing/prefer-importing-vitest-globals
```

```fixed
import { describe, it } from "vitest";

describe("a", () => {
  it("b", () => {});
});
```

### case: a member access is not a call of the global

```ts
const mock = vi.fn();
```

```expect
(none)
```

### case: a name imported from another module is not a global

```ts
import { it } from "./my-runner";

it("b", () => {});
```

```expect
(none)
```

### case: all imported

```ts
import { describe, expect, it } from "vitest";

describe("a", () => {
  it("b", () => {
    expect(1).toBe(1);
  });
});
```

```expect
(none)
```
