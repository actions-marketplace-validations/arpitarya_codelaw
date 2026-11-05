---
id: numpy/numpy-legacy-random
title: "A Generator, not np.random's legacy functions"
languages: [python]
fix: manual
fix_reason: "moving to a `Generator` means creating one and passing it where it is needed"
fix_exception: restructure
stability: preview
ported_from: [ruff/NPY002]
since: 0.5.0
---

Random numbers come from a `np.random.Generator` (`np.random.default_rng()`), not the legacy `np.random.*` functions.

**Why:** The legacy functions share hidden global state, which makes results depend on call order and hard to reproduce.

```codelaw
match:
  pattern: np.random.$F($$$A)
  inside: { kind: module, stopBy: end, has: { any: [{kind: import_statement}, {kind: import_from_statement}], regex: '^(import|from)\s+numpy\b' } }
constraints:
  F: { regex: '^(beta|binomial|bytes|chisquare|choice|dirichlet|exponential|f|gamma|geometric|get_state|gumbel|hypergeometric|laplace|logistic|lognormal|logseries|multinomial|multivariate_normal|negative_binomial|noncentral_chisquare|noncentral_f|normal|pareto|permutation|poisson|power|rand|randint|randn|random|random_integers|random_sample|ranf|rayleigh|sample|seed|set_state|shuffle|standard_cauchy|standard_exponential|standard_gamma|standard_normal|standard_t|triangular|uniform|vonmises|wald|weibull|zipf)$' }
message: "Use a `np.random.Generator` instead of `np.random.$F`"
```

## How to fix

1. Create `rng = np.random.default_rng(seed)` and call `rng.normal(...)` (and so on) on it.

## Examples

### Python

#### Don't

```python
import numpy as np
x = np.random.normal(size=3)
```

#### Do

```python
import numpy as np
rng = np.random.default_rng(0)
x = rng.normal(size=3)
```
