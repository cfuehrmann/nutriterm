# nutriterm

Terminal-based nutrition calculator for recipes. A Rust CLI application that calculates nutritional information for ingredients and recipes using a flexible JSON-based data system.

## Features

- **Calculate Nutrition** - Get detailed nutritional breakdown for any recipe including net carbs, protein, fat, fiber, and calories
- **Multiple Ingredients** - Create recipes with multiple ingredients and see combined nutritional values
- **Easy Recipe Management** - List all your recipes and view nutrition for specific ones
- **Smart Workspace** - Works from any directory - automatically finds your recipe data like git does
- **Human-Readable Format** - Uses JSONC (JSON with comments) so you can easily read and edit your data files

## Getting Started

### Installation

Download the latest release from GitHub or install directly from git:

```bash
cargo install --git https://github.com/cfuehrmann/nutriterm
```

### First Steps

1. **Create a workspace** in a new directory:
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
         "name": "chicken_breast",
         "display_name": "Chicken Breast",
         "carbs_per_100g": 0.0,
         "protein_per_100g": 31.0,
         "fat_per_100g": 3.6,
         "fiber_per_100g": 0.0
       },
       {
         "name": "brown_rice",
         "display_name": "Brown Rice (cooked)",
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
         "description": "Healthy protein and carbs meal",
         "ingredients": [
           {
             "name": "chicken_breast",
             "grams": 150
           },
           {
             "name": "brown_rice",
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
   
   150.0 g  Chicken Breast (skinless)
   100.0 g  Brown Rice (cooked)
   ```

### Daily Usage

```bash
# See all your recipes
nutriterm list-recipes

# Get nutrition for any recipe (exact name match)
nutriterm recipe "Grilled Chicken with Rice"

# Search with multiple terms (finds recipes containing ALL terms)
nutriterm recipe chicken rice  # Finds recipes with both "chicken" AND "rice" in name

# The tool works from anywhere in your recipe directory tree
cd subfolder
nutriterm list-recipes  # Still works!
```

### Tips

- **Net carbs** = Total carbs - Fiber (this is what's displayed)
- **Recipe search** uses the "name" field only, not "description" - search terms must ALL be found in the recipe name
- **Add comments** to your JSONC files to remember where you got nutritional data
- **Use descriptive names** like "Chicken Rice Bowl" rather than "recipe1" (use quotes in commands for names with spaces)

## Data Format Reference

Your recipe workspace contains two main files that you'll edit:

### `ingredients.jsonc` - Your Ingredient Database

This file contains nutritional information for all foods you use. Each ingredient needs:

```jsonc
{
  "ingredients": [
    {
      "name": "chicken_breast",           // Internal ID (used in recipes)
      "display_name": "Chicken Breast",   // What users see in output
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
      "description": "Healthy chicken bowl", // Shown when listing recipes  
      "ingredients": [
        {
          "name": "chicken_breast",         // Must match ingredient "name"
          "grams": 150                      // Amount in grams
        },
        {
          "name": "white_rice",
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
- `src/models/` - Core data structures (Ingredient, WeightedIngredient, Recipe)
- `src/data/loader.rs` - JSONC loading, parsing, and validation with comprehensive error handling
- `src/schema/generator.rs` - JSON schema generation for IDE autocompletion support
- `src/workspace.rs` - Workspace detection and validation logic
- `src/commands/` - Command implementations (init, recipe, list-recipes)
- `src/display/nutrition.rs` - Nutrition table formatting and display logic
- `src/error.rs` - Centralized error handling and user-friendly error messages

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

1. Run tests: `cargo test`
2. Check formatting: `cargo fmt --check`  
3. Check linting: `cargo clippy -- -D warnings`
4. Review snapshot changes: `cargo insta review`