---
trigger: always_on
---

# System Architecture Principles

1. **Scalability by Default:** The architecture should support rapid scaling without major rewrites.
2. **"Smart Defaults":** Configuration should be zero-to-minimal out of the box. Opinionated paths are preferred over complex setups.
3. **Decoupling:** Core logic must be decoupled from the delivery mechanism (CLI vs Web vs Editor Extension).
4. **Resiliency:** The system should degrade gracefully when external APIs or AI services fail.
