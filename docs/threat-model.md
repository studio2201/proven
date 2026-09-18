# Threat model — proven

## 1. Adversary
An attacker who can poison the build environment between source commit
and the published binary artifact. The source is honest; the bytes are not.
This includes a compromised CI runner, a hijacked maintainer PGP key, a
poisoned package cache, or a malicious artifact server in front of the
release pipeline.

## 2. Trust boundaries
We trust: the source tree at HEAD, the maintainer's commit signature
(if any), and `Cargo.lock`. We do not trust: any process that runs
between `git tag` and `release publish`, the artifact server, the
package cache, or the user's download path.

## 3. Out of scope
Proven does not defend against: a compromised source tree (i.e., the
attacker has commit access), a stolen maintainer account used *during*
development (Proven only catches divergence at the artifact layer), or
a targeted attack against the user's own toolchain (`rustc`, `git`).

## 4. Residual risk
Proven's reproducibility story depends on §16's `(host, rustc, lockfile)`
indexing. A binary that has never been built with the maintainer's actual
host triple cannot be reproduced-verified by Proven — Proven will report
"baseline missing" rather than "this is a forgery." Buyers should treat
a baseline-missing result as "not yet attested," not as "attested clean."
