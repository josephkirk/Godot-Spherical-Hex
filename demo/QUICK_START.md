# Quick Start Guide

This demo shows how to use the Spherical Hex Grid extension in your Godot project.

## Demo Scenes

1. `main.tscn` - Basic sphere with hex grid
2. `game_board.tscn` - Strategic game board example
3. `planet.tscn` - Planet generation example

## Running the Demo

1. Open the project in Godot
2. Load one of the demo scenes
3. Press Play

## Scene Setup Example

Here's how to set up a basic spherical hex grid:

```gdscript
# main.gd
extends Node3D

func _ready():
    var hex_grid = $SphericalHexGrid
    
    # Configure grid
    hex_grid.radius = 5.0
    hex_grid.resolution = 2
    hex_grid.hex_size = 0.1
    
    # Generate grid
    hex_grid.generate_grid()
    
    # Example: Color random hexes
    for hex in get_tree().get_nodes_in_group("hex_tiles"):
        if randf() > 0.5:
            color_hex(hex, Color(1, 0, 0))

func color_hex(hex, color):
    var material = StandardMaterial3D.new()
    material.albedo_color = color
    hex.get_node("Mesh").material_override = material
```

## Tips

1. Start with low resolution (1-2) while testing
2. Use `hex_grid.get_tile_at_position()` for interaction
3. Add meshes as children of hex tiles for custom visuals
4. Group hex tiles for easy access
