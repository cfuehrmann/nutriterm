# Repository Agent Instructions

## Development Commands

Run `just --list` to see all available commands.

- **Iterating on code:** `just test` - Fast feedback loop for logic changes
- **Before committing:** `just check` - All quality gates (fmt -> clippy -> spell -> test)
- **Fix formatting:** `just fix` - Auto-format code

## Core Practices (Priority Order)

1. **Write tests first** - Add comprehensive tests before implementing features
2. **Run `just check`** before committing to ensure all quality gates pass
3. **Follow PR workflow** - All changes to main branch must go through pull requests with CI validation
4. **NEVER commit automatically** - Only create commits when explicitly asked by the user
5. **Check for unused dependencies** when reviewing code changes
6. **Check module structure and organization** for coherence and proper separation of concerns
7. **Keep README.md up to date** - always verify README accuracy and completeness
8. **Review snapshot updates** with `cargo insta review` when needed

## Testing (Critical)

- **Integration tests over unit tests** - Test complete user workflows end-to-end
- **Vertical test slicing** - Each test represents one complete user story from start to finish
- **One command per test** - Never repeat commands within a single test
- **Platform-independent** - Code must work identically across all platforms
- **Snapshot testing** - Capture all user-facing output for regression prevention

## Code Quality

- **Comments**: Focus on "why" not "what" - avoid self-evident comments
- **Commit messages**: Clear, concise descriptions of changes and their purpose - avoid verbose details like file counts or line numbers
- **Documentation**: Keep README.md accurate and complete - verify it reflects current state
- **Commits**: NEVER create commits without explicit user request - always ask before committing

## Code Organization

- **Module structure**: Organize by domain/feature, not technical layer
- **Folder coherence**: Related functionality should be colocated
- **Clear boundaries**: Each module should have a single, well-defined responsibility
- **Minimize cross-dependencies**: Reduce coupling between modules
- **Consistent naming**: Module names should clearly indicate their purpose

---

<details>
<summary>Detailed Guidelines (Reference)</summary>

### Code Comments
- **Avoid "what" comments** that simply echo what the code does (e.g., `// Set x to 5`)
- **Focus on "why" comments** that explain business logic, edge cases, or non-obvious decisions
- **Remove self-evident comments** like `// Import models module` or `// Create a variable`
- **Keep comments concise** - prefer clear variable/function names over lengthy explanations

### README.md Maintenance

Always keep README.md accurate and complete. Update for any changes that affect:

**User-Facing Changes:**
- New commands or features → Update Features and Getting Started sections
- Command syntax changes → Update usage examples  
- New file formats → Update Data Format Reference
- Error messages or workflows → Update Troubleshooting

**Developer-Facing Changes:**
- New source files → Update Project Structure section
- Test changes → Update test count and descriptions
- Development workflow → Update Development commands

**Requirements:**
- User info first (Features → Getting Started → Data Format)
- Clear separation with `---` and "For Developers" heading
- All examples must work as shown
- Features describe user benefits, not implementation details
- Verify README accuracy during code reviews
- Update immediately when functionality changes

### Code Organization Details

**Module Structure:**
- **Domain-driven organization** - Group by business functionality (commands, models, data) rather than technical patterns (utils, helpers, managers)
- **Feature cohesion** - Related types, functions, and logic should live in the same module
- **Single responsibility** - Each module should have one clear purpose
- **Dependency direction** - Higher-level modules should depend on lower-level ones, not vice versa
- **Public API clarity** - Each module should expose a minimal, focused public interface

**Folder Structure:**
- **Logical grouping** - Directory structure should reflect the mental model of the domain
- **Avoid deep nesting** - Keep directory hierarchy simple and navigable
- **Consistent patterns** - Similar modules should follow the same organizational pattern
- **Colocation benefits** - Files that change together should be located near each other

**Colocation Best Practices:**
- **Tests near code** - Integration tests organized by feature, not by technical implementation
- **Related types together** - Data structures and their associated functions in the same module
- **Error handling colocation** - Error types near the code that uses them
- **Schema and validation** - Data schemas close to the models they validate
- **Avoid scattered concerns** - Don't spread a single feature across multiple distant modules

</details>
