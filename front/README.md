# Leptos Frontend with Tailwind CSS

This directory contains the frontend code for the Leptos project, styled with Tailwind CSS.

## Setup

1. Install Node.js dependencies:

```bash
cd front
npm install
```

2. Build the project with Trunk:

```bash
trunk build
```

Or for development with hot-reloading:

```bash
trunk serve
```

## Tailwind CSS

This project uses Tailwind CSS for styling. The configuration is set up as follows:

- `tailwind.css`: Contains the Tailwind directives (`@tailwind base`, `@tailwind components`, `@tailwind utilities`)
- `tailwind.config.js`: Configuration for Tailwind, including which files to scan for classes
- `package.json`: Contains scripts for building Tailwind CSS
- `build-tailwind.sh`: Shell script that ensures dependencies are installed before building Tailwind CSS

### How it works

1. The `build-tailwind.sh` script is executed during the Trunk build process (configured in `Trunk.toml`)
2. This script processes the `tailwind.css` file and generates a full CSS file with all the Tailwind styles in `dist/tailwind.css`
3. The `index.html` file includes:
   - A directive to copy the processed CSS file: `<link data-trunk rel="copy-file" href="dist/tailwind.css" />`
   - A link to load the CSS file in the browser: `<link rel="stylesheet" href="tailwind.css" />`

### Development Workflow

When developing, you can use the watch script to automatically rebuild Tailwind CSS when changes are made:

```bash
npm run watch
```

This will watch for changes to your Tailwind CSS configuration and rebuild the CSS file.

### Adding New Styles

To add new styles, simply add Tailwind classes to your components in the `src` directory. The classes will be automatically included in the final CSS file.

For example:

```rust
view! {
    <div class="bg-blue-500 text-white p-4 rounded-lg shadow-md">
        "Hello, world!"
    </div>
}
```

## Project Structure

- `src/`: Contains the Rust source code for the frontend
  - `components/`: Reusable UI components
  - `store/`: State management code
  - `jslike/`: JavaScript-like utilities
  - `main.rs`: Entry point for the application
- `public/`: Static assets
- `dist/`: Build output directory
- `index.html`: HTML template for the application
- `Trunk.toml`: Configuration for Trunk
- `tailwind.css`: Tailwind CSS entry point
- `tailwind.config.js`: Tailwind CSS configuration
- `package.json`: Node.js package configuration
- `build-tailwind.sh`: Script to build Tailwind CSS

## Troubleshooting

### Tailwind CSS Build Issues

If you encounter issues with the Tailwind CSS build process:

1. Make sure Node.js and npm are installed on your system:
   ```bash
   node --version
   npm --version
   ```

2. Try installing the dependencies manually:
   ```bash
   cd front
   npm install
   ```

3. Run the Tailwind CSS build script directly:
   ```bash
   ./build-tailwind.sh
   ```

4. If you see permission errors, make sure the script is executable:
   ```bash
   chmod +x build-tailwind.sh
   ```

### CSS Not Loading or Empty CSS File

If you see that the Tailwind CSS file is being referenced but it's empty or only contains the directives:

1. Check that the build script is generating the processed CSS file:
   ```bash
   cd front
   ./build-tailwind.sh
   ls -la ./dist/tailwind.css
   ```

2. Make sure your index.html file is correctly configured:
   ```html
   <!-- This tells Trunk to copy the processed CSS file -->
   <link data-trunk rel="copy-file" href="dist/tailwind.css" />

   <!-- This tells the browser to load the CSS file -->
   <link rel="stylesheet" href="tailwind.css" />
   ```

3. If you're still having issues, try rebuilding the project from scratch:
   ```bash
   cd front
   rm -rf dist
   trunk build
   ```
