# 🧩 Role of Each Module (quick reminder)

---

```aiignore
 ___________________________________________________________________________________
|     File      |                           Purpose                                 |                          |
|_______________|__________________________________________________________________ |
|   case.rs     | Logic for grouping facts into "cases", tracking changes over time |
|_______________|___________________________________________________________________|
|   search.rs   | Graph traversal & entity search logic (with filters, types, etc.) |
|_______________|___________________________________________________________________|
|   timeline.rs | Temporal queries, valid-time/transaction-time logic               |
|_______________|___________________________________________________________________|
|   utils.rs    | Helpers: timestamp parsers, sorting, deduplication, filters       |
|_______________|___________________________________________________________________|
|   mod.rs      | Public API of `engine`, re-exports submodules cleanly             |
|_______________|___________________________________________________________________|
```