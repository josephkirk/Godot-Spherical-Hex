# Spherical Hex Grid Implementation Guidelines

This document details the step-by-step implementation of the spherical hex grid system, explaining the mathematical concepts, algorithms, and code structure.

## Table of Contents
1. [System Overview](#system-overview)
2. [Mathematical Foundation](#mathematical-foundation)
3. [Core Components](#core-components)
4. [Implementation Steps](#implementation-steps)
5. [Optimization Techniques](#optimization-techniques)
6. [Common Challenges](#common-challenges)

## System Overview

The spherical hex grid system creates a hexagonal grid on a sphere surface using the following key components:

1. Icosahedron base for sphere approximation
2. Subdivision system for resolution control
3. Hex grid generation on faces
4. Face stitching and seam handling
5. Node-based representation in Godot

## Mathematical Foundation

### Coordinate Systems

1. Cube Coordinates for Hex Grid
   ```rust
   struct HexCoord {
       q: i32,  // x axis
       r: i32,  // y axis
       s: i32,  // z axis (derived: s = -q-r)
   }
   ```

2. Spherical Coordinates
   ```rust
   // Converting between Cartesian and Spherical
   fn cartesian_to_spherical(point: Vec3) -> Vec2 {
       let r = point.length();
       let theta = point.z.atan2(point.x);
       let phi = (point.y / r).acos();
       Vec2::new(theta, phi)
   }
   ```

### Icosahedron Construction

1. Golden Ratio Constants
   ```rust
   const PHI: f32 = 1.618033988749895;
   const PHI_NORM: f32 = 0.8506508083520399;  // 1/sqrt(1 + phi^2)
   const ONE_NORM: f32 = 0.5257311121191336;  // 1/sqrt(1 + phi^2)
   ```

2. Vertex Generation
   - 12 vertices based on golden ratio
   - 20 faces with equilateral triangles

## Core Components

### 1. IcosahedronFace
- Represents a triangular face
- Handles subdivision
- Maintains face orientation

```rust
struct IcosahedronFace {
    vertices: [Vec3; 3],
    normal: Vec3,
}
```

### 2. HexGrid
- Manages hex tile placement
- Handles neighbor relationships
- Coordinates face transitions

### 3. SphericalHexGrid Node
- Main Godot interface
- Controls generation parameters
- Manages tile instances

## Implementation Steps

### 1. Base Structure Setup
```
- src/
  ├── lib.rs           // Extension entry point
  ├── hex.rs           // Hex coordinate system
  ├── sphere_grid.rs   // Main node implementation
  ├── hex_grid.rs      // Grid generation logic
  └── math/
      ├── coordinates.rs
      └── projection.rs
```

### 2. Icosahedron Implementation
1. Generate base vertices
2. Create face connections
3. Implement subdivision algorithm
   ```rust
   fn subdivide(&self, depth: i32) -> Vec<IcosahedronFace> {
       if depth == 0 { return vec![self.clone()]; }
       
       // Get midpoints and create new faces
       let mid1 = (self.vertices[0] + self.vertices[1]).normalize();
       let mid2 = (self.vertices[1] + self.vertices[2]).normalize();
       let mid3 = (self.vertices[2] + self.vertices[0]).normalize();
       
       // Create and subdivide new faces
       ...
   }
   ```

### 3. Hex Grid Generation
1. Create hex coordinates
2. Project onto face planes
3. Handle face boundaries
   ```rust
   fn generate_on_face(&mut self, face: IcosahedronFace) {
       let center = face.center();
       let orientation = calculate_face_orientation(face);
       
       // Generate hex grid in 2D
       for q in -radius..=radius {
           for r in -radius..=radius {
               if abs(-q-r) <= radius {
                   generate_hex(q, r, center, orientation);
               }
           }
       }
   }
   ```

### 4. Seam Handling
1. Identify boundary hexes
2. Match hexes across faces
3. Merge duplicate vertices
   ```rust
   fn find_matching_hex(&self, hex: &HexCoord, face: &IcosahedronFace) -> Option<HexCoord> {
       // Find nearest hex on adjacent face
       let world_pos = hex_to_world_position(hex);
       let nearest = find_nearest_hex_on_face(world_pos, face);
       
       if is_within_merge_threshold(world_pos, nearest) {
           Some(nearest)
       } else {
           None
       }
   }
   ```

### 5. Node System Integration
1. Create hex tile nodes
2. Set up transforms
3. Handle parent-child relationships
   ```rust
   fn create_hex_tile(&mut self, coord: HexCoord, position: Vec3) {
       let mut tile = HexTile::new();
       tile.set_transform(calculate_transform(position));
       add_child(tile);
       connect_neighbors(tile);
   }
   ```

## Optimization Techniques

### 1. Spatial Partitioning
- Divide sphere into sectors
- Use quadtree for hex lookup
- Cache neighbor relationships

### 2. Batch Processing
- Group hex creation
- Batch transform updates
- Use instance pools

### 3. LOD System
```rust
struct LODLevel {
    resolution: i32,
    visible_distance: f32,
}

fn update_lod(&mut self, camera_position: Vec3) {
    for tile in tiles {
        let distance = tile.position.distance(camera_position);
        tile.set_lod_level(get_appropriate_lod(distance));
    }
}
```

## Common Challenges

### 1. Pole Handling
- Problem: Hexes distort at poles
- Solution: Special pole cap handling
  ```rust
  fn generate_pole_cap(&mut self, is_north: bool) {
      let pole_position = if is_north { Vec3::Y } else { -Vec3::Y };
      let radius = calculate_pole_radius();
      generate_special_pole_hexes(pole_position, radius);
  }
  ```

### 2. Seam Artifacts
- Problem: Visible gaps or overlaps
- Solution: 
  1. Use overlap threshold
  2. Merge boundary vertices
  3. Adjust UV coordinates

### 3. Performance Scaling
- Problem: Exponential growth with resolution
- Solutions:
  1. Implement LOD
  2. Use spatial partitioning
  3. Chunk loading/unloading

## Testing Guidelines

### 1. Unit Tests
```rust
#[test]
fn test_hex_neighbor_consistency() {
    let grid = HexGrid::new(settings);
    let hex = HexCoord::new(0, 0);
    let neighbors = grid.get_neighbors(hex);
    
    // Each hex should have 6 neighbors
    assert_eq!(neighbors.len(), 6);
    
    // Test neighbor relationships are bidirectional
    for neighbor in neighbors {
        assert!(grid.get_neighbors(neighbor).contains(&hex));
    }
}
```

### 2. Visual Tests
1. Check seam visibility
2. Verify hex regularity
3. Test pole appearance
4. Validate neighbor connections

## Performance Benchmarks

Track these metrics during development:
1. Generation time vs resolution
2. Memory usage per hex
3. Frame time impact
4. LOD transition smoothness

## Extension Points

The system can be extended with:
1. Custom hex data components
2. Biome generation
3. Path finding systems
4. Resource management
5. Planet generation

## Best Practices

1. Always normalize vectors after operations
2. Cache frequently accessed data
3. Use appropriate data structures
   - HashMap for sparse data
   - Vec for dense data
4. Handle edge cases explicitly
5. Document mathematical operations
6. Unit test coordinate conversions

## Resource Management

### 1. Memory Considerations
```rust
struct HexTilePool {
    active_tiles: HashMap<HexCoord, HexTile>,
    inactive_tiles: Vec<HexTile>,
    max_tiles: usize,
}
```

### 2. GPU Resources
- Use instance rendering
- Batch mesh updates
- Implement occlusion culling

## Debugging Tools

1. Visual Debug Helpers
   - Face boundaries
   - Hex coordinates
   - Neighbor connections

2. Performance Monitors
   - Generation time
   - Memory usage
   - Frame time impact

## Appendix

### A. Mathematical Reference
1. Hex grid formulas
2. Sphere projection equations
3. Coordinate conversion matrices

### B. Optimization Tips
1. Profile before optimizing
2. Cache expensive calculations
3. Use appropriate data structures
4. Implement LOD early

### C. Code Style Guidelines
1. Use descriptive names
2. Document complex algorithms
3. Separate concerns
4. Write unit tests
