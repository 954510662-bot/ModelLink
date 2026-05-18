# ModelLink Visual Identity System Design Guide

## 1. Design Concept & Inspiration

### 1.1 Core Concept
ModelLink is a local proxy tool that enables AI coding tools to transparently use any third-party model. Its core value propositions are:
- **Link**: Connect different AI models and services
- **Proxy**: Act as an intermediate layer for protocol conversion
- **Transparent**: Completely transparent to upper-layer applications
- **Local**: Emphasize data privacy and local deployment

### 1.2 Design Inspiration
Logo design is inspired by **neural network connection diagrams** and **network topologies**:

- **Center Node**: Represents ModelLink as the core proxy/hub
- **Connection Lines**: Symbolize communication links and protocol conversion between models
- **Satellite Nodes**: Represent different AI models or services
- **Diagonal Connections**: Suggest complex many-to-many connection relationships

This design embodies technical attributes (network, links) while maintaining sufficient abstraction and artistry.

### 1.3 Design Goals
- **Simple & Professional**: Easy to recognize and remember
- **Technically Strong**: Reflect AI/technology attributes
- **Highly Distinctive**: Unique and not easily confused with other brands
- **Flexibly Adaptable**: Support multiple sizes and backgrounds

---

## 2. Color Specifications

### 2.1 Primary Color Palette

| Color Name | Value | Usage |
|-----------|-------|-------|
| Gradient Start | `#667eea` | Primary color, representing innovation and technology |
| Gradient End | `#764ba2` | Secondary color, representing connection and collaboration |
| Gradient Effect | `linear-gradient(135deg, #667eea, #764ba2)` | Main brand color |

### 2.2 Background Color Palette

| Color Name | Value | Usage |
|-----------|-------|-------|
| Dark Background | `#0f0f23` | GitHub dark mode, formal scenarios |
| Secondary Dark | `#1a1a2e` | Card backgrounds, secondary containers |
| Light Background | `#ffffff` | Light mode, documentation |

### 2.3 Text Color Palette

| Usage | Light Background | Dark Background |
|------|-----------------|-----------------|
| Main Text | `#1a1a2e` | `#ffffff` |
| Secondary Text | `#6b7280` | `#a0a0b0` |
| Logo Text | Gradient | Gradient |

### 2.4 Gradient Application Guide
```css
/* Main brand gradient */
background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);

/* Logo text */
background: linear-gradient(135deg, #667eea, #764ba2);
-webkit-background-clip: text;
-webkit-text-fill-color: transparent;
```

---

## 3. Typography Specifications

### 3.1 Recommended Font Stack
```css
font-family: 'Segoe UI', -apple-system, BlinkMacSystemFont,
             'Helvetica Neue', Arial, sans-serif;
```

### 3.2 Font Size Hierarchy

| Usage | Size | Weight | Notes |
|------|------|--------|-------|
| Logo Text | 28-32px | 700 (Bold) | Main display |
| Heading H1 | 24px | 600 | Page title |
| Heading H2 | 20px | 600 | Section title |
| Body Text | 16px | 400 | Main content |
| Helper Text | 14px | 400 | Explanatory text |
| Small Text | 12px | 400 | Labels, comments |

### 3.3 Usage Notes
- Logo text must use bold (700)
- Avoid overly decorative fonts
- Ensure consistent display across various operating systems

---

## 4. Logo Design Specifications

### 4.1 Icon Design

#### Structural Composition
- **Center Node**: Radius 1/4 of icon size
- **Main Connection Lines**: 4 lines connecting center to four directions
- **Secondary Connection Lines**: 4 lines connecting corner nodes
- **Nodes**: 8 total (4 large + 4 small)

#### Visual Proportions
```
Icon size: 64x64 (base)
- Center node: r=8
- Direction nodes: r=4
- Diagonal nodes: r=3
- Line width: 2px (main), 1.5px (secondary)
```

### 4.2 Safe Area
Sufficient whitespace should be preserved around the logo. Recommendations:
- Minimum safe margin: 1/4 of icon width
- Recommended margin: 1/2 of icon width

### 4.3 Minimum Usage Size
| Scenario | Minimum Size |
|---------|-------------|
| Favicon | 16x16 |
| Mobile Lists | 24x24 |
| Document Icons | 32x32 |
| Website Icons | 64x64 |
| Marketing Materials | 128x128+ |

---

## 5. File Inventory

### 5.1 SVG Files (Vector, Lossless Scaling)

| Filename | Size | Description |
|---------|------|-------------|
| `modelink-logo-icon.svg` | 64x64 | Logo icon |
| `modelink-logo-full.svg` | 240x64 | Logo + text combination |

### 5.2 PNG Files (Bitmap, Suitable for Various Scenarios)

| Filename | Size | Usage |
|---------|------|-------|
| `modelink-logo-icon-512.png` | 512x512 | Large display |
| `modelink-logo-icon-256.png` | 256x256 | Medium size |
| `modelink-logo-icon-128.png` | 128x128 | Icon usage |
| `modelink-logo-icon-64.png` | 64x64 | Small icon |
| `modelink-logo-icon-32.png` | 32x32 | Compact display |
| `modelink-logo-icon-16.png` | 16x16 | Favicon |

### 5.3 Background Images

| Filename | Size | Description |
|---------|------|-------------|
| `modelink-background-1920x1080.png` | 1920x1080 | GitHub repository cover |

### 5.4 Tool Files

| Filename | Description |
|---------|-------------|
| `visual-identity-generator.html` | Interactive preview generator |
| `generate-assets.ps1` | PowerShell batch generation script |
| `DESIGN_GUIDE.md` | Design specification document |

---

## 6. Usage Scenario Guide

### 6.1 GitHub Repository
- **Repository Cover**: Use `modelink-background-1920x1080.png`
- **Repository Icon**: Use `modelink-logo-icon-64.png`
- **README Header**: Use `modelink-logo-full.svg` or PNG version

### 6.2 Web Applications
- **Navigation Logo**: Use SVG or 128x128 PNG
- **Favicon**: Use 16x16 or 32x32 PNG
- **Loading Animation**: Can use logo rotation or pulse effects

### 6.3 Documentation Materials
- **Title Page**: Use complete logo (icon + text)
- **Section Icons**: Use icon-only version
- **Footer**: Use small size icon

### 6.4 Presentation Materials
- **PPT/Keynote**: Use SVG or large PNG (512x512+)
- **Business Cards**: Use 300dpi PNG
- **Merchandise**: Contact designer for specific dimensions

---

## 7. Dark & Light Modes

### 7.1 Dark Mode (Recommended)
- Background: `#0f0f23` or `#1a1a2e`
- Logo: Use gradient version (maintain consistency)
- Text: White `#ffffff`

### 7.2 Light Mode
- Background: `#ffffff` or `#f9fafb`
- Logo Text: Use `#1a1a2e` or gradient
- Icon: Maintain gradient colors

---

## 8. Important Notes

### 8.1 Prohibited Actions
- Do not stretch or distort the logo
- Do not change the gradient angle
- Do not replace brand colors
- Do not add shadows around the logo (except in design specifications)
- Do not place the logo on low contrast backgrounds

### 8.2 Recommended Practices
- Always maintain aspect ratio
- Use the provided safe margins
- Test visibility on different backgrounds
- Preview on mobile devices

---

## 9. Update History

| Version | Date | Updates |
|---------|------|---------|
| 1.0 | 2026-05-19 | Initial version with logo and background image |

---

## 10. Contact Information

For further custom design or questions, please contact the project maintainers.
