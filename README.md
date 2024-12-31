# Spherical Hex Grid for Godot

A GDExtension that generates hexagonal grids on spheres, perfect for planet-based games, strategic games, or any game requiring spherical hex-based geography.

## Features

- Generate hexagonal grids on spherical surfaces
- Automatic subdivision based on resolution settings
- Proper hex neighbor relationships
- Customizable grid size and hex size
- Efficient hex lookup and navigation
- Full GDScript API support
- Built with Rust for optimal performance

## Requirements

- Godot 4.1 or later
- Rust toolchain (for building from source)
- Cargo (Rust's package manager)

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

Each hex is a node that you can interact with:

```gdscript
# Get hex at world position
var hex = hex_grid.get_tile_at_position(Vector3(1, 0, 0))

# Get hex coordinates
var coords = hex.get_coordinate()

# Get neighbors
var neighbors = hex.get_neighbors()
```

### Customizing Hex Appearance

The HexTile node is a Node3D, so you can add your own meshes and materials:

```gdscript
# In your scene setup
var hex_tile = hex_grid.get_tile_at_position(position)
var mesh_instance = MeshInstance3D.new()
mesh_instance.mesh = your_hex_mesh
hex_tile.add_child(mesh_instance)
```

## Advanced Features

### Face-based Generation

The grid is generated using an icosahedron-based approach:

1. Base icosahedron is created
2. Faces are subdivided based on resolution
3. Hex grid is generated on each face
4. Boundaries are stitched together
5. Vertices are projected onto sphere

### Neighbor Relationships

Hexes maintain proper neighbor relationships even across face boundaries:

```gdscript
# Get all connected hexes
var neighbors = hex.get_neighbors()

# Check if hexes are neighbors
var is_neighbor = hex1.is_neighbor(hex2)
```

## Examples

The extension includes a demo scene showing various use cases:

1. Basic planet generation
2. Strategic game board
3. Path finding example
4. Custom hex styling

## API Reference

### SphericalHexGrid

Properties:
- `radius: float` - Sphere radius
- `resolution: int` - Subdivision level
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
- `get_coordinate() -> Vector3` - Get hex coordinate
- `is_neighbor(other: HexTile) -> bool` - Check if hexes are neighbors

## Contributing

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Known Issues

1. High resolution (>5) can be performance intensive
2. Seams might be visible at very low resolutions
3. Memory usage increases exponentially with resolution

## Roadmap

- [ ] GPU-accelerated generation
- [ ] LOD support
- [ ] Runtime modification tools
- [ ] Texture coordinate generation
- [ ] Biome generation helpers

## Support

For questions and support:
1. Check the [issues](https://github.com/yourusername/spherical-hex/issues) page
2. Create a new issue if needed
3. Join our Discord community (link coming soon)
