# Spherical Hex Grid for Godot

A GDExtension that generates hexagonal grids on spheres, perfect for planet-based games, strategic games, or any game requiring spherical hex-based geography.

## Features

- Generate hexagonal grids on spherical surfaces using icosahedron subdivision
- Automatic subdivision based on resolution settings
- Proper hex neighbor relationships across face boundaries
- Customizable grid size and hex size
- Efficient hex lookup and navigation
- Full GDScript API support
- Built with Rust for optimal performance
- Memory-efficient node management

## Requirements

- Godot 4.1 or later
- Rust toolchain (stable channel)
- Cargo (Rust's package manager)
- godot-rust/gdext (master branch)

## Installation

1. Clone this repository into your Godot project's `addons` folder:
   ```bash
   cd your_godot_project
   git clone https://github.com/yourusername/spherical-hex.git addons/spherical_hex
   ```

2. Build the extension:
   ```bash
   cd addons/spherical_hex
   cargo build --release
   ```

3. Enable the plugin in Godot:
   - Open Project Settings
   - Go to Plugins tab
   - Find "Spherical Hex Grid" and enable it

## Usage

### Basic Setup

1. Add a SphericalHexGrid node to your scene
2. Configure the grid properties:
   ```gdscript
   var hex_grid = $SphericalHexGrid
   hex_grid.radius = 5.0       # Sphere radius
   hex_grid.resolution = 2     # Subdivision level (higher = more hexes)
   hex_grid.hex_size = 0.1     # Size of individual hexes
   ```

3. Generate the grid:
   ```gdscript
   hex_grid.generate_grid()
   ```

### Working with Hex Tiles

Each hex is a Node3D with additional properties:

```gdscript
# Get hex at world position
var hex = hex_grid.get_tile_at_position(Vector3(1, 0, 0))

# Get hex coordinates (q, r, s)
var coords = hex.get_coordinate()

# Get neighboring hexes
var neighbors = hex.get_neighbors()
```

### Customizing Hex Appearance

HexTiles automatically create a basic visual representation, but you can customize it:

```gdscript
# Accessing a hex tile
var hex_tile = hex_grid.get_tile_at_position(position)

# Customize scale
hex_tile.transform.basis = hex_tile.transform.basis.scaled(Vector3(1.1, 0.2, 1.1))

# Add custom mesh
var mesh_instance = MeshInstance3D.new()
mesh_instance.mesh = your_hex_mesh
hex_tile.add_child(mesh_instance)
```

## Implementation Details

### Grid Generation Process

1. Icosahedron Creation
   - Generates base 20-face icosahedron
   - Each face is an equilateral triangle

2. Face Subdivision
   - Recursively subdivides faces based on resolution
   - Maintains proper neighbor relationships
   - Projects vertices onto sphere surface

3. Hex Grid Generation
   - Generates hex grid on each subdivided face
   - Handles face boundaries and pole regions
   - Creates efficient neighbor relationships

### Technical Features

- Memory-efficient node management
- Proper coordinate system across face boundaries
- Optimized neighbor lookups
- Automatic cleanup and regeneration

## API Reference

### SphericalHexGrid

Properties:
- `radius: float` - Sphere radius
- `resolution: int` - Subdivision level (0-5 recommended)
- `hex_size: float` - Size of hexes

Methods:
- `generate_grid()` - Generate/regenerate the grid
- `get_tile_at_position(pos: Vector3) -> HexTile` - Get hex at world position
- `set_radius(radius: float)` - Set sphere radius
- `set_resolution(res: int)` - Set subdivision level
- `set_hex_size(size: float)` - Set hex size

### HexTile

Properties:
- `coordinate: Vector3` - Hex grid coordinate (q,r,s)

Methods:
- `get_neighbors() -> Array[HexTile]` - Get neighboring hexes
- `get_coordinate() -> Vector3` - Get hex coordinates
- `set_coordinate(q: int, r: int)` - Set hex coordinates

## Development

### Code Organization
```
addons/spherical_hex/
├── src/
│   ├── lib.rs               - Extension entry point
│   ├── hex.rs              - Hex tile implementation
│   ├── sphere_grid.rs      - Main grid implementation
│   ├── hex_grid.rs         - Grid generation logic
│   └── math/
│       ├── coordinates.rs   - Coordinate systems
│       └── projection.rs    - Sphere projection
├── docs/
│   ├── implementation_guideline.md
│   └── reminder.md
├── tests/
├── Cargo.toml
└── plugin.cfg
```

## Known Issues

1. High resolution (>5) increases memory usage significantly
2. Seams might be visible at very low resolutions (0-1)
3. Performance scales with face count (resolution^2)

## Performance Guidelines

1. Start with low resolution (2-3) for testing
2. Adjust hex_size based on sphere radius
3. Consider using LOD for large spheres
4. Monitor memory usage at high resolutions

## Roadmap

- [ ] GPU-accelerated generation
- [ ] Dynamic LOD system
- [ ] Runtime modification tools
- [ ] UV coordinate generation
- [ ] Biome generation system
- [ ] Path finding helpers
- [ ] Serialization support

## Support

For questions and support:
1. Check the [issues](https://github.com/yourusername/spherical-hex/issues) page
2. Review the documentation in `/docs`
3. Create a new issue if needed

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

1. Fork the repository
2. Create your feature branch
3. Follow the implementation guidelines in `/docs`
4. Submit a pull request

The codebase follows Rust best practices and the gdext extension patterns. See `/docs/reminder.md` for common patterns and gotchas.