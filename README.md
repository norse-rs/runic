# Runic

`Runic` is a playground for testing out different path rendering approaches with focus on anti-aliasing quality on low dpi displays.


## Usage

```
cargo run --example furu --release
```
#### Keys
- `F1` - Coarse rasterizer (box filter)
- `F2` - Distance rasterizer (box filter)
- `F3` - Coarse rasterizer (1x1 Heaviside filter)
- `F4` - Coarse rasterizer (8x8 Heaviside filter) (`B` for Tent filter, `N` for Box filter)
- `F5` - Analytic rasterizer (box filter)
- `1` - Default scene (two triangles)

```
cargo run --example filters --release
```
#### Keys
- `B` - Tent filter
- `N` - Box filter
