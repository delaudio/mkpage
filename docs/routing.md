# Content routing

mkpage discovers `.md` and `.html` files below the configured source directory.
Files and directories beginning with `.` or `_`, and all symbolic links, are
ignored. Discovery order is lexicographic and does not depend on filesystem
enumeration order.

Routes are directory-style URLs:

| Content path | Route | Output |
| --- | --- | --- |
| `index.md` | `/` | `index.html` |
| `about.md` | `/about/` | `about/index.html` |
| `projects/index.md` | `/projects/` | `projects/index.html` |
| `projects/mkpage.md` | `/projects/mkpage/` | `projects/mkpage/index.html` |

Every source path is validated before an output path is calculated. Traversal,
absolute/prefixed paths, unsafe separators, and encoded traversal or separators
are rejected with the offending candidate and reason. Exact and case-only route
collisions are errors on every platform. A static asset is also rejected when it
would overwrite a generated page output.
