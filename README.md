# nutriterm

Terminal-based nutrition calculator for recipes. A Rust CLI application that calculates nutritional information for ingredients and recipes using a flexible JSON-based data system.

## Features

- **Calculate Nutrition** - Get detailed nutritional breakdown for any recipe including net carbs, protein, fat, fiber, and calories
- **Multiple Ingredients** - Create recipes with multiple ingredients and see combined nutritional values
- **Recipe Analysis** - View detailed nutrition for any recipe with smart search
- **Kitchen Reference** - Generate a printable HTML reference with all recipes and ingredient weights
- **Smart Catalog Discovery** - Works from any directory - automatically finds your recipe data like git does
- **Human-Readable Format** - Uses JSONC (JSON with comments) so you can easily read and edit your data files

## Getting Started

### Installation

Download the latest release from GitHub or install directly from git:

```bash
cargo install --git https://github.com/cfuehrmann/nutriterm
```

### First Steps

1. **Create a catalog** in a new directory:
   ```bash
   mkdir my-recipes
   cd my-recipes
    nutriterm init
   ```

2. **Edit your ingredient data** - Open `ingredients.jsonc` and add nutritional information for foods you use:
   ```jsonc
   {
     "ingredients": [
       {
         "id": "chicken_breast",
         "name": "Chicken Breast",
         "carbs_per_100g": 0.0,
         "protein_per_100g": 31.0,
         "fat_per_100g": 3.6,
         "fiber_per_100g": 0.0
       },
       {
         "id": "brown_rice",
         "name": "Brown Rice (cooked)",
         "carbs_per_100g": 23.0,
         "protein_per_100g": 2.6,
         "fat_per_100g": 0.9,
         "fiber_per_100g": 1.8
       }
     ]
   }
   ```

3. **Create your first recipe** - Edit `recipes.jsonc` to add a recipe:
   ```jsonc
   {
     "recipes": [
        {
          "name": "Grilled Chicken with Rice",
          "ingredients": [
           {
             "id": "chicken_breast",
             "grams": 150
           },
           {
             "id": "brown_rice",
             "grams": 100
           }
         ]
       }
     ]
   }
   ```

4. **View nutrition information**:
   ```bash
    nutriterm recipe "Grilled Chicken with Rice"
   ```
   
   **Example output:**
   ```
   Recipe: Grilled Chicken with Rice
   
   ╭───────────────────────────┬──────────┬─────────────┬───────────┬───────┬─────────┬────────────╮
   │  Name                     │  Weight  │  Net carbs  │  Protein  │  Fat  │  Fiber  │  Calories  │
   ├───────────────────────────┼──────────┼─────────────┼───────────┼───────┼─────────┼────────────┤
   │ Chicken Breast (skinless) │  150.0 g │         0 g │    46.5 g │ 5.4 g │     0 g │   235 kcal │
   │       Brown Rice (cooked) │  100.0 g │      23.0 g │     2.6 g │ 0.9 g │   1.8 g │   110 kcal │
   │                     Total │  250.0 g │      23.0 g │    49.1 g │ 6.3 g │   1.8 g │   345 kcal │
    ╰───────────────────────────┴──────────┴─────────────┴───────────┴───────┴─────────┴────────────╯
    ```

### Daily Usage

```bash
# Get nutrition for any recipe (exact name match)
nutriterm recipe "Grilled Chicken with Rice"

# Search with multiple terms (finds recipes containing ALL terms)
nutriterm recipe chicken rice  # Finds recipes with both "chicken" AND "rice" in name

# Generate kitchen reference with all recipes (great for printing)
nutriterm kitchen-ref

# The tool works from anywhere in your recipe directory tree
cd subfolder
nutriterm recipe "My Recipe"  # Still works!
```

### Kitchen Reference

Generate a clean HTML reference of all your recipes with ingredient weights:

```bash
nutriterm kitchen-ref > kitchen-reference.html
```

The HTML output is self-contained and works great for viewing in browsers. To create a PDF, simply open the HTML file in your browser and print to PDF.

**Example output:**
```html
<!DOCTYPE html>
<html>
<head><title>Kitchen Reference</title></head>
<body>
<h1>Kitchen Reference</h1>

<h2>Chicken Rice Bowl</h2>
<ul>
<li>150.0 g  Chicken Breast (skinless)</li>
<li>100.0 g  Brown Rice (cooked)</li>
<li>80.0 g  Broccoli (steamed)</li>
</ul>

<h2>Greek Salad</h2>
<ul>
<li>100.0 g  Mixed Greens</li>
<li>50.0 g  Feta Cheese</li>
<li>75.0 g  Cherry Tomatoes</li>
<li>60.0 g  Cucumber</li>
</ul>
</body>
</html>
```

This is perfect for:
- **Printing** for kitchen reference while cooking
- **Shopping lists** when you know the recipes you want to make
- **Recipe sharing** in a clean, readable format

### Tips

- **Net carbs** = Total carbs - Fiber (this is what's displayed)
- **Recipe search** uses the "name" field - search terms must ALL be found in the recipe name
- **Add comments** to your JSONC files to remember where you got nutritional data
- **Use descriptive names** like "Chicken Rice Bowl" rather than "recipe1" (use quotes in commands for names with spaces)

## Data Format Reference

Your recipe catalog contains two main files that you'll edit:

### `ingredients.jsonc` - Your Ingredient Database

This file contains nutritional information for all foods you use. Each ingredient needs:

```jsonc
{
  "ingredients": [
    {
      "id": "chicken_breast",            // Stable ID (used in recipes)
      "name": "Chicken Breast",          // Display name (shown to users)
      "carbs_per_100g": 0.0,             // Carbohydrates per 100g
      "protein_per_100g": 31.0,          // Protein per 100g  
      "fat_per_100g": 3.6,               // Fat per 100g
      "fiber_per_100g": 0.0              // Fiber per 100g (subtracted from carbs)
    }
    // Add more ingredients...
  ]
}
```

**Where to find nutritional data**: USDA food database, nutrition labels, or apps like MyFitnessPal.

### `recipes.jsonc` - Your Recipe Collection

This file defines your recipes using ingredients from the database:

```jsonc
{
  "recipes": [
    {
      "name": "Chicken Rice Bowl",           // Used in commands (requires quotes)  
        "ingredients": [
          {
            "id": "chicken_breast", // Must match ingredient "id"
            "grams": 150                       // Amount in grams
          },
          {
            "id": "white_rice",
            "grams": 100
          }
        ]
    }
    // Add more recipes...
  ]
}
```

**Tip**: Use descriptive names like "Chicken Rice Bowl" for better readability (remember to use quotes in commands).

---

## For Developers

The following sections contain information for developers working on this project.

### Architecture & Quality

This project emphasizes:
- **Robust Error Handling** - Clear, actionable error messages with helpful suggestions  
- **Comprehensive Testing** - Integration tests with snapshot validation for reliable behavior
- **Platform Independence** - Works identically across all operating systems
- **Performance** - Zero-allocation design for recipe calculations

### Development

#### Running Tests

```bash
cargo test
```

#### Checking with Clippy

```bash
cargo clippy -- -D warnings
```

Ensure there are no linter warnings before committing.

#### Formatting Code

```bash
cargo fmt
```

### Project Structure

#### Source Code

- `src/main.rs` - CLI argument parsing and application coordination
- `src/catalog/` - Recipe catalog operations (read-only data access)
  - `items/` - Core data structures (Ingredient, WeightedIngredient, Recipe)
  - `jsonc/` - JSONC file handling, schema generation, and initialization
  - `discovery.rs` - Catalog directory detection and validation logic
  - `mod.rs` - Catalog module coordination and public API
- `src/commands/` - Command implementations (init, recipe, kitchen-ref)
  - `recipe/` - Recipe command with search and nutrition display
- `src/error/` - Centralized error handling with semantic error types
- `src/utils/` - Utility functions (suggestions, etc.)

#### Tests

- `tests/cli.rs` - Integration tests with snapshot testing using `insta`
- **Comprehensive tests** covering all user scenarios and error cases
- **Vertical test slicing** - each test covers one complete user journey
- **Platform-independent** - no OS-specific testing techniques
- **Snapshot validation** - captures all user-facing output for regression prevention

## Testing Approach

This project uses **snapshot testing** with the `insta` crate to ensure consistent CLI behavior across all user scenarios. Tests focus on integration testing of complete user workflows rather than unit testing.

### Testing Principles

- **Integration over unit tests** - Test complete user journeys end-to-end
- **One command per test** - Each test executes exactly one command with unique preconditions
- **Comprehensive coverage** - Every user-facing feature and error condition is tested
- **Platform independence** - All tests work identically across operating systems

### Working with Insta

```bash
# Install the insta CLI tool
cargo install cargo-insta

# Review snapshots interactively (recommended)
cargo insta review

# Accept all pending snapshots (use with caution)
cargo insta accept

# Run tests and show any snapshot diffs
cargo insta test
```

For more information, visit the [insta documentation](https://insta.rs/docs/).

## Contributing

### Quality Gates

All changes to the main branch must go through pull requests and pass automated quality gates:

- **Tests**: `cargo test` - All 37 tests must pass
- **Formatting**: `cargo fmt --check` - Code must be properly formatted  
- **Linting**: `cargo clippy -- -D warnings` - No lint warnings allowed
- **Build**: `cargo check` and `cargo build --release` - Must build successfully

### Development Workflow

1. **Create feature branch**: `git checkout -b feature/your-change`
2. **Make changes** and ensure quality gates pass locally:
   ```bash
   cargo test
   cargo fmt --check
   cargo clippy -- -D warnings
   cargo insta review  # for snapshot changes
   ```
3. **Create pull request**: All CI checks must pass before merging
4. **Automated checks**: GitHub Actions runs all quality gates
5. **Merge**: Only possible when all required status checks pass

### Branch Protection

The main branch is protected with:
- ✅ Required status checks (tests, formatting, linting, build)
- ✅ No direct pushes allowed - PRs only
- ✅ Strict status checks - branches must be up to date
- ✅ Applies to administrators