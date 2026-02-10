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

64 files | 8,027 lines

| Language | Files | Lines |
|----------|------:|------:|
| Rust | 54 | 5,933 |
| JavaScript | 3 | 32 |
| JSON | 3 | 678 |
| Text | 2 | 2 |
| HTML | 1 | 1,357 |
| TOML | 1 | 25 |

**Largest files:**
- `src/api/report.rs` (1,418 lines)
- `debug_report.html` (1,357 lines)
- `src/i18n/legacy.rs` (1,174 lines)
- `src/report/html.rs` (800 lines)
- `translations/en.json` (226 lines)
- `translations/es.json` (226 lines)
- `translations/pt-br.json` (226 lines)
- `src/pptx_mapper.rs` (151 lines)
- `examples/probe_th.rs` (133 lines)
- `src/plugins/builtin/google_slides.rs` (124 lines)