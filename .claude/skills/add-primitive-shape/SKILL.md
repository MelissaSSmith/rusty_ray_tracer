---
name: add-primitive-shape
description: >-
  Step-by-step guide for adding a new primitive shape (geometry type) to this
  Rust ray tracer. Use when implementing a new Shape — e.g. sphere/cube/cone/
  disk-like geometry with its own ray-intersection and surface-normal math.
  Covers the shape-module template, every Shape-dispatch touch-point in
  shape.rs, the object-space intersection/normal conventions, the critical
  `transformed_ray` pitfall (which unit tests do NOT catch), bounding boxes,
  and the unit-test + render validation workflow.
---

# Adding a new primitive shape

Geometry in this ray tracer is **not** trait objects. `Shape` (`src/features/shapes/shape.rs`)
is an enum of every primitive kind; `Object` is the renderable wrapper that holds the
transform matrices, material, shadow flag, and bounding box. Per-shape behavior is
dispatched by `match` in `Object`'s `Intersect`/`NormalAt` impls.

A new primitive does its math in **object space**: it assumes it sits at the canonical
origin (e.g. a unit sphere at `(0,0,0)`, a cylinder around the y-axis). The renderer feeds
it a ray that has already been transformed into that space, and transforms the resulting
normal back to world space. You implement the canonical-space geometry; transforms come for
free **as long as you wire the dispatch correctly** (see the pitfall below).

See `CLAUDE.md` for the wider architecture. The cleanest existing primitives to copy from:
`sphere.rs` (stateless), `cylinder.rs` / `cone.rs` (parameters + Object accessors),
`disk.rs` / `capsule.rs` (parameters read by extracting the struct from the enum).

## Files you touch

1. **`src/features/shapes/<name>.rs`** — new module: struct + builders + `Intersect` + `Normal` + tests.
2. **`src/features/shapes.rs`** — one line: `pub mod <name>;`.
3. **`src/features/shapes/shape.rs`** — the dispatch. 5–7 edits (below).
4. **`tests/<name>_test.rs`** — an `#[ignore]`d render test (validation; do **not** skip).

## Step 1 — the shape module (`src/features/shapes/<name>.rs`)

Template (adapt the math; this shows the required shape of the file). Derive `Copy` — it is
required so `shape.rs` can write `Shape::<Name>(*x)`.

```rust
use crate::features::intersection::Intersection;
use crate::features::primitives::operations::consts::EPSILON; // if you need an epsilon test
use crate::features::primitives::point::Point;
use crate::features::primitives::tuple_trait::Tuple;          // brings .x()/.y()/.z()
use crate::features::primitives::vector::Vector;
use crate::features::ray::Ray;
use crate::features::shapes::{Intersect, Normal};
use crate::features::shapes::shape::{Object, Shape};          // Shape only if you extract the struct

#[derive(Clone, Copy, Debug)]
pub struct Widget {
    radius: f64,
}

impl Widget {
    pub fn create() -> Self { Widget { radius: 1.0 } }
    pub fn radius(&self) -> f64 { self.radius }
    pub fn with_radius(self, radius: f64) -> Self { Self { radius, ..self } }
}

impl Intersect for Widget {
    // `_ray` is ALREADY in object space (see shape.rs dispatch). Solve the canonical-space
    // geometry. Return every root; the world sorts globally and Intersection::hit() picks
    // the visible one, so don't filter on t-sign (match plane.rs).
    fn intersect(_object: &Object, _ray: &Ray) -> Vec<Intersection> {
        // Option A: read params via Object accessors (cylinder/cone/torus idiom).
        // Option B: pull your struct straight out of the enum (disk/capsule idiom) —
        //           use this when you need fields that have no Object accessor.
        let widget = match _object.shape() {
            Shape::Widget(w) => w,
            _ => return vec![],
        };
        let _ = (widget.radius(), EPSILON); // ... real intersection math here ...
        vec![/* Intersection::create(t, _object, 0.0, 0.0), ... */]
    }
}

impl Normal for Widget {
    // `_point` is the hit point already converted to OBJECT space. Return the object-space
    // normal (need not be normalized — normal_to_world normalizes it).
    fn normal(_object: &Object, _point: &Point) -> Vector {
        Vector::create(_point.x(), 0.0, _point.z())
    }
}

#[cfg(test)]
mod tests {
    use crate::features::primitives::point::Point;
    use crate::features::primitives::tuple_trait::Tuple;
    use crate::features::primitives::vector::Vector;
    use crate::features::ray::Ray;
    use crate::features::shapes::widget::Widget;
    use crate::features::shapes::{Intersect, Normal};
    use crate::features::shapes::shape::Shape;

    #[test]
    fn test_ray_hits_widget() {
        let widget = Shape::Widget(Widget::create()).create();
        let ray = Ray::create(Point::create(0.0, 0.0, -5.0), Vector::create(0.0, 0.0, 1.0));
        let xs = Widget::intersect(&widget, &ray);
        assert_eq!(xs.len(), 2);
    }
}
```

If the normal depends on barycentric `u`/`v` (like `smooth_triangle.rs`), implement
`NormalAt` instead of `Normal` (`fn normal(_object, _point, _hit) -> Vector`).

## Step 2 — register the module (`src/features/shapes.rs`)

Add alongside the other `pub mod` lines:

```rust
pub mod widget;
```

## Step 3 — wire the dispatch (`src/features/shapes/shape.rs`)

All of these are in `shape.rs`. Most other `match self.shape()` sites in the codebase have a
`_ =>` fallback, so they don't need editing — but these do:

1. **Import** the struct: `use crate::features::shapes::widget::Widget;`
2. **Enum variant** — carry the struct if it has parameters:
   ```rust
   pub enum Shape { /* ... */ Widget(Widget), /* ... */ }
   ```
   (Stateless shapes like `Sphere`/`Cube` are bare variants; data-carrying ones wrap a struct.)
3. **`Shape::as_str`** arm: `Shape::Widget(_) => "Widget",`
4. **`Shape::create_object`** arm — build the object-space `BoundingBox`:
   ```rust
   Shape::Widget(w) => {
       let bounds = BoundingBox::create()
           .with_minimum(Point::create(-w.radius(), -w.radius(), -w.radius()))
           .with_maximum(Point::create(w.radius(), w.radius(), w.radius()));
       Object::create(Shape::Widget(*w), has_shadow, material, bounds)
   }
   ```
   For radial shapes there's a helper: `Object::create_radial_bounds(center, radius, height)`.
   The material presets (`.create()`, `.glass()`, `.water()`, …) and `with_transform` /
   `with_material` work automatically once this arm exists — no other edits needed.
5. **`impl Intersect for Object`** arm — ⚠️ pass **`&transformed_ray`**, not `_ray`:
   ```rust
   Shape::Widget(_) => { Widget::intersect(_object, &transformed_ray) }
   ```
6. **`impl NormalAt for Object`** arm — `object_point` is already world→object transformed:
   ```rust
   Shape::Widget(_) => { Widget::normal(_object, &object_point) }
   ```
7. **`Object` accessors (only if you chose Option A above)** — if your `intersect`/`normal`
   read `_object.radius()` / `minimum_bound()` / `center()` / `height()` etc., add an arm for
   your shape in those methods (they delegate via `match self.shape()` with a `_ => 0.0`/`zero`
   fallback that would otherwise silently feed you the wrong value). Option B (extracting the
   struct) avoids this entirely.

## ⚠️ The pitfall that unit tests will NOT catch

In the `Intersect for Object` dispatch, object-space primitives **must** receive
`&transformed_ray` (the ray pushed into object space by the inverse transform). Only
*containers/self-positioning* shapes take the raw `_ray` (`Group`, `CSG`, and the triangle
types, which apply transforms internally / bake them into vertices).

If you pass `_ray` by mistake, the primitive ignores its own transform and renders at the
world origin. **Your object-space unit tests still pass**, because they use an identity
transform where `transformed_ray == _ray`. This exact bug shipped in `Disk` and `Torus`.

The renderer transforms the ray **without renormalizing direction**, which is deliberate: it
keeps the `t` parameter identical between world and object space, so the object-space `t` you
return is valid for the world-space hit point. Don't normalize the direction yourself, and
make `a = direction · direction` (not `1`) if your quadratic needs it — see `sphere.rs`.

## Conventions / gotchas

- **Operators on tuples/vectors are non-standard** (`tuple.rs`): `^` is the **dot** product,
  `*` between two tuples is the **cross** product, `==` is **epsilon** equality. For raw
  `f64`, use the `Operations` trait: `a.equals(b)`, `a.zero()`. Epsilons live in
  `primitives::operations::consts` (`EPSILON = 1e-5`, `LOW_EPSILON = 1e-4`).
- **`#[derive(Clone, Copy, Debug)]`** on the struct — `Copy` is required for `Shape::Widget(*w)`.
- **Don't name a getter `normal()`** if you also `impl Normal` — the inherent method and the
  trait method collide and `Widget::normal(obj, pt)` won't resolve. Use `normal_vector()`
  (the `triangle.rs` / `disk.rs` convention).
- **Return all intersections; let the world filter.** `World::intersect` sorts globally and
  `Intersection::hit()` keeps the first `t > LOW_EPSILON`. Returning behind-the-ray (negative
  `t`) hits is fine and matches `plane.rs`.

## Step 4 — validate (mandatory)

```sh
cargo test --lib widget::     # the inline unit tests (object space)
cargo test --lib              # full suite — confirms the new enum variant broke no dispatch
```

Then **render through a non-trivial transform** — this is the only thing that catches the
`transformed_ray` pitfall. Add `tests/<name>_test.rs` mirroring `tests/torus_test.rs` /
`tests/capsule_test.rs` (walls + the shape, `#[ignore]`d), place the shape with a
`Matrix::translate(...) * Matrix::rotate_*(...)`, then:

```sh
cargo test --test widget_test -- --ignored
python ppm_to_png.py widget_test.ppm widget_test.png   # bundled with this skill
```

The bundled `ppm_to_png.py` parses the P3 ASCII PPM directly (the locally-available Pillow is
too old to read P3) and writes a PNG (plus a `_720` thumbnail) you can open/inspect. Confirm
the shape appears **at its transformed position** with correct shading — not at the origin.

## Quick checklist

- [ ] `src/features/shapes/<name>.rs`: struct (`Clone,Copy,Debug`) + builders + `Intersect` + `Normal` + tests
- [ ] `src/features/shapes.rs`: `pub mod <name>;`
- [ ] `shape.rs`: import · enum variant · `as_str` · `create_object`+bounds · `intersect` (**`&transformed_ray`**) · `normal` · accessors (if Option A)
- [ ] `cargo test --lib` green
- [ ] render through a translate+rotate and eyeball the PNG
