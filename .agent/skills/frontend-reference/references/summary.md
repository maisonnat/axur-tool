This file is a merged representation of a subset of the codebase, containing files not matching ignore patterns, combined into a single document by Repomix.
The content has been processed where comments have been removed, content has been formatted for parsing in markdown style, content has been compressed (code blocks are separated by ⋮---- delimiter).

# Summary

## Purpose

This is a reference codebase organized into multiple files for AI consumption.
It is designed to be easily searchable using grep and other text-based tools.

## File Structure

This skill contains the following reference files:

| File | Contents |
|------|----------|
| `project-structure.md` | Directory tree with line counts per file |
| `files.md` | All file contents (search with `## File: <path>`) |
| `tech-stack.md` | Languages, frameworks, and dependencies |
| `summary.md` | This file - purpose and format explanation |

## Usage Guidelines

- This file should be treated as read-only. Any changes should be made to the
  original repository files, not this packed version.
- When processing this file, use the file path to distinguish
  between different files in the repository.
- Be aware that this file may contain sensitive information. Handle it with
  the same level of security as you would the original repository.
- Pay special attention to the Repository Description. These contain important context and guidelines specific to this project.
- Pay special attention to the Repository Instruction. These contain important context and guidelines specific to this project.

## Notes

- Some files may have been excluded based on .gitignore rules and Repomix's configuration
- Binary files are not included in this packed representation. Please refer to the Repository Structure section for a complete list of file paths, including binary files
- Files matching these patterns are excluded: **/*.lock, **/target/**, **/node_modules/**, **/dist/**, **/.git/**, **/*.png, **/*.jpg, **/*.ico, **/*.svg
- Files matching patterns in .gitignore are excluded
- Files matching default ignore patterns are excluded
- Code comments have been removed from supported file types
- Content has been formatted for parsing in markdown style
- Content has been compressed - code blocks are separated by ⋮---- delimiter
- Files are sorted by Git change count (files with more changes are at the bottom)
- Git diffs from the worktree and staged changes are included

## Statistics

44 files | 4,912 lines

| Language | Files | Lines |
|----------|------:|------:|
| Rust | 38 | 2,244 |
| JavaScript | 2 | 4 |
| TOML | 2 | 47 |
| No Extension | 1 | 5 |
| HTML | 1 | 2,612 |

**Largest files:**
- `index.html` (2,612 lines)
- `src/api.rs` (372 lines)
- `src/pages/editor.rs` (313 lines)
- `src/pages/dashboard.rs` (284 lines)
- `src/pages/logs.rs` (113 lines)
- `src/storage.rs` (111 lines)
- `src/pages/apply.rs` (80 lines)
- `src/pages/login.rs` (79 lines)
- `src/pages/admin_beta.rs` (74 lines)
- `src/lib.rs` (60 lines)