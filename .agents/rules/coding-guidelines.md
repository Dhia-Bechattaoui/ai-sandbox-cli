---
trigger: always_on
---

# Coding Guidelines

1. **Type Safety:** Always use strict typing (TypeScript, Rust, Go). No implicit `any`.
2. **Modularity:** Keep functions small and single-purpose. Separate business logic from UI/CLI logic.
3. **Error Handling:** Never swallow errors. Provide actionable, human-readable error messages.
4. **Performance:** Prioritize speed and low memory usage. Avoid blocking the main thread.
5. **Documentation:** Comment complex logic. Maintain up-to-date READMEs and docstrings.
