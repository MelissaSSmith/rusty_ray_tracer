# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project overview

A CPU ray tracer written in Rust, built incrementally following Jamis Buck's *The Ray Tracer Challenge*. Most modules and the integration tests map directly to chapters/scenes from the book (projectile, analog clock, spheres & planes, glamour shots, OBJ meshes, cubes/cylinders/cones, groups, CSG, etc.), and the codebase is heavily test-driven.

The crate is **both a library and a binary**:
- `src/lib.rs` exposes the `features` and `draw` modules; integration tests in `tests/` consume it as `rusty_ray_tracer::...`.
- `src/main.rs` is a small CLI. The only implemented command renders an OBJ file with a default world/camera:
  ```
  cargo run --release -- create_object <path/to/model.obj>
  ```
  It writes `<model>.ppm` next to the working directory. Camera/light/view setup is hard-coded in `main.rs` and `src/world_builder.rs`.

(`ray_tracer_ui/` is a stale, empty scaffold — no `Cargo.toml` or source. Ignore it.)

## Build / test commands

```sh
cargo build                 # debug build
cargo build --release       # release (use this for any real rendering — renders are slow)
cargo test                  # runs all UNIT tests (inline #[cfg(test)] modules) + non-ignored integration tests
cargo test <name>           # filter by test name substring
cargo test --test world     # run a single integration-test file (tests/world.rs)
```

**Scene-rendering integration tests are marked `#[ignore]`** because they actually render and write multi-MB `.ppm` files (slow). They do **not** run under a plain `cargo test`. To run them:

```sh
cargo test -- --ignored                          # run all ignored scene tests
cargo test --test torus_test -- --ignored        # run one scene test file
```

`Cargo.toml` sets `[profile.test] opt-level = 2` so even debug test runs are optimized — necessary because the scene tests are compute-heavy.

CI (`.github/workflows/`) runs `cargo build --verbose` and `cargo test --verbose` on push/PR to `main` — i.e. it only exercises unit tests and the non-ignored integration tests, never the `#[ignore]`d renders.

Generated artifacts (`*.ppm`, `*.xcf`) and the OBJ scratch files are gitignored.

## Architecture

### The `Shape` / `Object` model (most important to understand)

Geometry is **not** done with trait objects. Instead:

- `Shape` (`src/features/shapes/shape.rs`) is an enum of every primitive kind (`Sphere`, `Plane`, `Cube`, `Cylinder`, `Cone`, `Group`, `Triangle`, `SmoothTriangle`, `CSG`, `Torus`, `Disk`, ...). Variants that carry data hold a per-shape struct (e.g. `Cone(Cone)`); parameterless primitives are bare variants.
- `Object` is the concrete, renderable wrapper around a `Shape`. It owns the material, shadow flag, bounding box, and **two pairs of transform matrices**: the local `transformation`/`inverse_transformation` and the `cumulative_transform`/`cumulative_inverse_transform` (used for group/CSG hierarchies).
- Per-shape behavior is dispatched by `match` in `impl Intersect for Object` and `impl NormalAt for Object`, which delegate to each shape's own `Intersect`/`Normal`/`NormalAt` impl. **When you add a new primitive**, you touch: the `Shape` enum, `Shape::as_str`, `Shape::create_object` (bounds), and the `match` arms in `Object::intersect` and `Object::normal`.

### Construction idiom: consuming builders

Objects are created from a `Shape` and then refined with chained, **`self`-consuming** builder methods that return a new `Object`:

```rust
let s = Shape::Sphere.create()                       // -> Object (identity transform, default material)
    .with_transform(Matrix::translate(-0.5, 1.0, 0.5))
    .with_material(some_material);
```

`Shape` also has material-preset constructors: `.create()`, `.glass()`, `.water()`, `.diamond()`, `.air()`, `.vacuum()`. Most types follow the same `Type::create(...)` + `.with_xxx(...)` convention (`Camera`, `World`, `Material`, patterns, lights, etc.). There are also a few mutating `set_xxx` methods (e.g. `Object::set_transform`, `Camera::set_transform`).

### Transforms, groups, CSG, and the BVH

- `Matrix` provides static transform constructors (`Matrix::identity`, `translate`, `scale`, `rotate_x/y/z`, `view_transform`) and composes left-to-right via `*`. `Matrix * point/vector` applies the transform.
- `with_transform`/`with_children` propagate transforms down `Group` and `CSG` trees and recompute cumulative transforms and parent-space bounding boxes. Normals/points cross object↔world space via `world_to_object` / `normal_to_world` using the **cumulative** inverse transform.
- `Object::divide(threshold)` builds a BVH-like hierarchy by recursively partitioning group children into sub-groups using bounding boxes — call it on large groups (e.g. parsed OBJ meshes) before rendering to speed up intersection.

### Rendering pipeline

`Camera::render(world)` (`src/features/camera.rs`) → `World::color_at(ray)` (`src/features/world.rs`):

1. The camera shoots one ray per pixel. Rendering is **parallel via rayon** (`par_iter_mut` over the canvas).
2. `World::intersect` gathers and sorts all intersections; `Intersection::hit` picks the visible one.
3. `prepare_computations` builds a `Computation` (point, eye/normal vectors, over/under points, refractive indices `n1`/`n2`, reflect vector).
4. `shade_hit` applies Phong lighting (`Material::lighting`) plus **recursive** reflection and refraction, combined with Schlick reflectance for transparent+reflective surfaces. Recursion is bounded by `World::recursion_limit` (default 5).

### Canvas & output

`Canvas` (`src/features/canvas.rs`) stores pixels in a `DashMap<String, Color>` keyed by `"x.y"` strings — this concurrent map is what lets `Camera::render` write pixels in parallel. `src/draw/ppm_format.rs` serializes a canvas to PPM (`canvas.convert_to_ppm_and_save("name.ppm")`); `src/draw/obj_format.rs` parses Wavefront OBJ files into a `Group` object.

### Patterns & lights

Both are closed enums (`Patterns` in `src/features/patterns.rs`, `Light` in `src/features/lights.rs`) wrapping per-variant structs, dispatched by `match` — same design as `Shape`. Patterns carry their own transform; lights are `PointLight`, `AreaLight`, `SpotLight` and expose `intensity_at(point, world)` for soft/hard shadows.

## Conventions & gotchas

- **Operator overloading on `Tuple`/`Point`/`Vector` is non-standard** (`src/features/primitives/tuple.rs`):
  - `a * b` between tuples is the **cross product**.
  - `a ^ b` (`BitXor`) is the **dot product**. You will see `eye_vector ^ normal` everywhere — that is a dot product, not XOR.
  - `a * f64` is scalar multiplication.
- **Float comparison is epsilon-based.** `PartialEq` (`==`, and therefore `assert_eq!`) for `Tuple`/`Point`/`Vector`/`Color` compares with tolerance `EPSILON = 0.00001`, so tests assert against values rounded to ~4 decimals (e.g. `Vector::create(0.2857, 0.4286, -0.8571)`). For raw `f64` use the `Operations` trait: `a.equals(b)`, `a.equals_low_epsilon(b)`, `a.zero()`. Epsilon constants live in `src/features/primitives/operations/operations::consts` (`EPSILON`, `LOW_EPSILON = 0.0001`, `HIGH_EPSILON`). Don't introduce bare `==` on floats.
- **Tests are the spec.** Each source module carries inline `#[cfg(test)] mod tests`; integration scene tests live in `tests/`. New features should come with tests mirroring the existing book-driven style, and the expected numeric values typically come straight from the book.
- Dependencies of note: `rayon` (parallel render), `dashmap` (concurrent canvas), `noise` (perturbed patterns), `smallvec`, `rand`/`rand_distr` (area-light sampling).
