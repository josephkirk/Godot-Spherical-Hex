# Common Godot-Rust Development Reminders

## Node Tree Management

### Node Parenting
```rust
// ❌ Wrong
child.reparent(parent_node.upcast());

// ✅ Correct
child.reparent(&parent_node.upcast::<Node>());
```

Key points:
- Always pass references to reparent
- Specify concrete type when upcasting: `upcast::<Node>()`
- Wrap node tree operations in unsafe blocks

### Node Access
```rust
// ❌ Wrong
self.base.get_owner()
self.base.get_scene()

// ✅ Correct
self.base.to_gd()
```

## Transform & Basis Operations

### Creating Transforms
```rust
// ❌ Wrong
Transform3D::from_basis_origin(basis, origin)

// ✅ Correct
Transform3D::new(basis, origin)
```

### Basis Creation
```rust
// ❌ Wrong
Basis::from_euler(EulerRot::XYZ, angles)  // Using glam's EulerRot

// ✅ Correct
Basis::from_euler(EulerOrder::XYZ, angles)  // Using Godot's EulerOrder
```

### Scale Operations
```rust
// Simple scaling
Basis::from_diagonal(x, y, z)
```

## Type Safety

### Upcasting
```rust
// ❌ Wrong
node.upcast()  // Missing type specification
node.upcast::<Base>()  // Using generic parameter

// ✅ Correct
node.upcast::<Node>()  // Specific base class
```

## Common Patterns

### Node Tree Manipulation
```rust
unsafe {
    let parent_node = self.base.to_gd();
    child.reparent(&parent_node.upcast::<Node>());
}
```

### Transform Setup
```rust
let transform = Transform3D::new(
    Basis::from_diagonal(scale_x, scale_y, scale_z),
    Vector3::new(pos_x, pos_y, pos_z)
);
node.set_transform(transform);
```

## Type Conversions

### Godot to glam Vec3
```rust
let godot_vec = Vector3::new(x, y, z);
let glam_vec = Vec3::new(godot_vec.x, godot_vec.y, godot_vec.z);
```

### glam to Godot Vector3
```rust
let glam_vec = Vec3::new(x, y, z);
let godot_vec = Vector3::new(glam_vec.x, glam_vec.y, glam_vec.z);
```

## Best Practices

1. Always use explicit type specifications for upcasting
2. Wrap node tree operations in unsafe blocks
3. Use Godot types (EulerOrder, etc.) when working with Godot objects
4. Use glam types for mathematical operations
5. Convert between Godot and glam types explicitly
6. Follow consistent patterns across codebase

## Common Errors to Watch For

1. Missing references in node operations
2. Using glam types where Godot types are expected
3. Forgetting unsafe blocks for node tree operations
4. Missing type specifications in upcasts
5. Using non-existent Godot methods (check API documentation)

## Debugging Tips

1. Check type specifications first when seeing upcast errors
2. Verify reference vs. value semantics when seeing borrow checker errors
3. Confirm unsafe block usage for node operations
4. Validate Godot vs glam type usage
5. Review API documentation for correct method names

## Performance Considerations

1. Batch node tree operations where possible
2. Cache transform and basis calculations
3. Minimize type conversions between Godot and glam
4. Use appropriate data structures for lookups (HashMap for sparse data)

## Documentation References

1. [Godot Rust Documentation](https://godot-rust.github.io/)
2. [Godot Engine Documentation](https://docs.godotengine.org/)
3. [glam-rs Documentation](https://docs.rs/glam/)

Remember to check these references when implementing new features or debugging issues.
