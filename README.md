# Simple Rust CPU raytracer

A simple, homebrewed, CPU-powered path tracer. It's not meant to be good path tracer, It's meant to teach me Rust.

## Controls

Movement: W A S D LShift Space

Orientation: Mouse

Zoom: Z X

Focus distance: J L

Aperture size: I M

Toggle autofocus: N

Toggle overlays: U

Toggle depth pass: Enter

## Preview

![alt text](https://i.imgur.com/Y5f9IJl.png)

## Missing features / To do / Wishlist

#### Optimization:

- Bounding volume hierarchy
- Precomputed blue noise sample mask
- Cache optimization
- Micro optimizations

#### UI:

- Options menu
- Improved Bresenham implementation
- Possibly switch frame buffer implementation

#### Raytracing features:

- Spectral rendering
- Lens effects:
  - Logitudinal achromatic / chromatic aberration
  - Lateral chromatic aberration
  - Spherical aberration
  - Customizable bokeh shapes
- GPU post-processing effects:
  - Bloom
- Scrambled Sobol
- Multiple importance sampling
- Bidirectional path tracing
- Refraction

#### Stretch goals:

- Metropolis light transport
  - (Automatic parameter control)
- Customizable reflectance distribution functions
- Polygon support
- Translucency
- Subsurface Scattering
- Proper PBR implementation
- Textures
- Normal mapping
