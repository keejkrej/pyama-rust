# Dioxus Primitives Component Guide

This document summarizes the patterns and structure learned from analyzing the dioxus primitives components in `/vendor/components/preview/`.

## Component Architecture

### Directory Structure
Each component follows this consistent pattern:
```
components/{component_name}/
├── docs.md              # Component documentation
└── variants/
    └── main/             # Primary variant (additional variants possible)
        ├── mod.rs        # Rust component implementation
        └── style.css     # Component-specific styles
```

### Component Implementation Pattern

#### 1. Basic Component Structure
```rust
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        // CSS inclusion via asset macro
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/{component}/variants/main/style.css"),
        }

        // Component markup
        div {
            class: "component-class",
            "data-variant": "primary",
            // Component content
        }
    }
}
```

#### 2. CSS Integration
- Each component includes its CSS using `document::Link` with `asset!()` macro
- CSS files are component-scoped and self-contained
- Styles use CSS custom properties for theming

### Styling System

#### CSS Variables (Theme System)
The components use a consistent CSS variable system for theming:

```css
:root {
  /* Primary colors */
  --primary-color: #ffffff;
  --primary-color-2: #f8fafc;
  --primary-color-3: #f1f5f9;
  --primary-color-4: #e2e8f0;
  --primary-color-5: #cbd5e1;
  --primary-color-6: #94a3b8;

  /* Secondary colors */
  --secondary-color-1: #0f172a;
  --secondary-color-2: #1e293b;
  --secondary-color-3: #334155;
  --secondary-color-4: #475569;

  /* Error colors */
  --primary-error-color: #dc2626;
  --secondary-error-color: #b91c1c;
  --contrast-error-color: #ffffff;

  /* Focus colors */
  --focused-border-color: #4f46e5;

  /* Light/dark mode support */
  --light: 1;
  --dark: 0;
}
```

#### Data Attributes for Variants
Components use `data-*` attributes to control styling variants:

```rust
button {
    class: "button",
    "data-style": "primary",  // or "secondary", "destructive", "outline", "ghost"
    "Button Text"
}
```

```css
.button[data-style="primary"] {
  background-color: var(--secondary-color-2);
  color: var(--primary-color);
}

.button[data-style="secondary"] {
  background-color: var(--primary-color-5);
  color: var(--secondary-color-1);
}
```

## Available Components

### Basic Components
- **button** - Interactive buttons with multiple variants
- **input** - Form input fields
- **label** - Form labels
- **separator** - Visual dividers

### Layout Components
- **accordion** - Collapsible content sections
- **collapsible** - Toggle content visibility
- **tabs** - Tabbed interface
- **toolbar** - Action toolbars

### Overlay Components
- **dialog** - Modal dialogs
- **popover** - Contextual pop-up content
- **tooltip** - Hover information
- **dropdown_menu** - Menu dropdowns
- **alert_dialog** - Confirmation dialogs

### Form Components
- **checkbox** - Boolean input controls
- **radio_group** - Single selection from multiple options
- **select** - Dropdown selection
- **slider** - Range input controls
- **switch** - Toggle switches

### Data Display
- **avatar** - User profile images
- **calendar** - Date selection (with simple variant)
- **progress** - Progress indicators
- **scroll_area** - Scrollable content areas

### Navigation
- **menubar** - Application menu bars
- **navbar** - Navigation bars

### Feedback
- **toast** - Notification messages

### Specialized
- **aspect_ratio** - Maintain element proportions
- **context_menu** - Right-click menus
- **hover_card** - Hover-triggered cards
- **toggle_group** - Multiple toggle controls

## Implementation Guidelines

### 1. Component Creation
1. Create directory structure following the pattern
2. Implement `Demo` component in `mod.rs`
3. Create corresponding `style.css` with CSS variables
4. Include CSS using `document::Link` and `asset!()` macro

### 2. Styling Best Practices
- Use CSS custom properties for all colors and spacing
- Implement variants using `data-*` attributes
- Ensure hover, focus, and active states
- Support light/dark theme switching via CSS variables

### 3. Component Registration
Components are registered using a macro system:
```rust
examples!(
    accordion,
    alert_dialog,
    button,
    // ... other components
);
```

### 4. Theme Customization
Override CSS variables to customize the theme:
```css
:root {
  --primary-color: #your-color;
  --secondary-color-1: #your-color;
  /* ... other overrides */
}
```

## Usage Examples

### Button Component
```rust
use dioxus::prelude::*;

#[component]
pub fn MyComponent() -> Element {
    rsx! {
        // Include button styles
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/button/variants/main/style.css"),
        }
        
        // Primary button
        button {
            class: "button",
            "data-style": "primary",
            onclick: |_| println!("Clicked!"),
            "Click me"
        }
        
        // Secondary button
        button {
            class: "button",
            "data-style": "secondary",
            "Cancel"
        }
    }
}
```

### Input Component
```rust
input {
    class: "input",
    placeholder: "Enter your name",
    value: "{name}",
    oninput: move |e| name.set(e.value()),
}
```

## Migration Strategy

1. **Copy component files** from `/vendor/components/preview/src/components/`
2. **Adapt paths** in asset macro calls to match your project structure
3. **Customize CSS variables** to match your design system
4. **Create component library module** to organize and export components
5. **Update components incrementally** replacing existing styled elements

## Notes

- All components are self-contained with their own CSS
- The system supports both light and dark themes via CSS variables
- Components follow semantic HTML patterns for accessibility
- Focus states and keyboard navigation are built-in
- The macro system allows for easy component registration and variant management