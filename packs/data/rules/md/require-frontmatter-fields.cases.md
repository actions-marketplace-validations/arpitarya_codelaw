# md/require-frontmatter-fields — cases

### case: silent without the option

```md
# No front matter
```

```expect
(none)
```

### case: no front matter

```md options={"fields":["title"]}
# Setup
```

```expect
1:1 md/require-frontmatter-fields
```

### case: a field is missing

```md options={"fields":["title","description"]}
---
title: Setup
---

# Setup
```

```expect
1:1 md/require-frontmatter-fields
```

### case: every field is there

```md options={"fields":["title","description"]}
---
title: Setup
description: Install it
---

# Setup
```

```expect
(none)
```

### case: a field with no value is missing

```md options={"fields":["title","description"]}
---
title: Setup
description:
---

# Setup
```

```expect
1:1 md/require-frontmatter-fields
```

### case: a list value counts

```md options={"fields":["tags"]}
---
tags:
  - a
---

# Setup
```

```expect
(none)
```

### case: TOML front matter

```md options={"fields":["title","description"]}
+++
title = "Setup"
+++

# Setup
```

```expect
1:1 md/require-frontmatter-fields
```

### case: front matter that is not at the top is not front matter

```md options={"fields":["title"]}
# Setup

---
title: A
---
```

```expect
1:1 md/require-frontmatter-fields
```
