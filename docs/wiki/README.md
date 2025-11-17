# Wiki Documentation

This directory contains comprehensive wiki documentation for the N-Body Galaxy Collision Simulation project.

## Contents

All documentation files are written in Markdown with Mermaid diagrams for visualization:

- **[Home.md](Home.md)** - Wiki home page with project overview and quick links
- **[Architecture.md](Architecture.md)** - System architecture with block diagrams and component flows
- **[Communication-Protocol.md](Communication-Protocol.md)** - WebSocket protocol with sequence diagrams
- **[Server-Components.md](Server-Components.md)** - Detailed server architecture and module documentation
- **[Client-Components.md](Client-Components.md)** - WASM client and WebGL rendering details
- **[Development-Guide.md](Development-Guide.md)** - Build, run, and debug instructions
- **[Shared-Library.md](Shared-Library.md)** - Common data structures and message types
- **[Configuration.md](Configuration.md)** - Configuration options and tuning guide

## Using These Files

### Option 1: Upload to GitHub Wiki

1. Go to your GitHub repository
2. Click the "Wiki" tab
3. Click "Create the first page" if wiki doesn't exist
4. Upload each .md file:
   - Click "New Page"
   - Copy the content from each file
   - Use the filename (without .md) as the page title
   - Save

**Note:** GitHub Wiki automatically supports Mermaid diagrams, so all diagrams will render correctly.

### Option 2: Use as Local Documentation

Simply browse the Markdown files in any Markdown viewer that supports Mermaid:
- VS Code with Markdown Preview Enhanced
- GitHub Desktop
- Typora
- Mark Text

### Option 3: Generate Static Site

Use a static site generator:
```bash
# Using mdBook
mdbook init wiki
cp *.md wiki/src/
mdbook build wiki
```

## Diagram Support

All documentation includes Mermaid diagrams for:
- Architecture block diagrams
- Sequence diagrams for message flows
- Component diagrams
- State diagrams
- Data flow diagrams
- Class diagrams

GitHub Wiki and most modern Markdown viewers automatically render Mermaid diagrams.

## Updating Documentation

When updating the documentation:

1. Edit the relevant .md file in this directory
2. Test Mermaid diagrams at https://mermaid.live/
3. Commit changes to this repository
4. Update the corresponding GitHub Wiki page

## Navigation

Each wiki page includes:
- Table of contents for easy navigation
- Links to related pages
- Back to Home link at the bottom

## Contributing

When adding new documentation:
1. Follow the existing structure and style
2. Include Mermaid diagrams where appropriate
3. Link to related pages
4. Update Home.md with links to new pages
