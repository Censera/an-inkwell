# Workflow

Development is done one TODO item at a time.

## 1. Select

Take the first unchecked item in `documentation/todo.md`.

Do not implement later items early. Do not add abstractions because a later item may need them.

## 2. Inspect

Read the relevant documentation and current implementation before changing code.

Check the existing ownership, lifetime, error, and LLVM handle model. Reuse the current structure unless the item exposes a real structural problem.

## 3. Implement

Implement only the selected item.

Keep the public API concrete and small. Keep raw LLVM handles private. Add no unrelated cleanup, refactoring, or speculative architecture.

## 4. Test

Add or update tests for the item before considering it complete.

Tests should cover the normal result and relevant failure or boundary cases. Test LLVM behavior rather than relying only on generated IR text when LLVM is allowed to constant-fold or otherwise canonicalize the result.

## 5. Validate

Before completion:

1. Check the ownership and lifetime relationships.
2. Run `cargo fmt`.
3. Run `cargo test`.
4. Fix every compiler error and warning.
5. Check that no unresolved TODO remains for the selected item.

Local test results are required before the item is marked complete.

## 6. Record

Only after validation passes, mark the selected item complete in `documentation/todo.md`.

The TODO file records completed work, not planned work that merely has code written for it.

## 7. Commit

Create one intentional commit for the completed item.

The commit message should describe the completed decision or capability. Avoid vague messages such as `changes`, `update`, or `fix stuff`.

## 8. Continue

After the commit, return to step 1 and take the next unchecked item.

A failing test, ownership problem, or compiler warning stops the workflow. Do not advance the TODO until the current item is actually complete.

## Completion rule

A TODO item is complete only when its implementation, tests, ownership model, formatting, compiler output, and local test suite are all clean and the result is recorded in its own intentional commit.
