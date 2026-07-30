# Shader & Filter Glossary — Impressive Visualisations

**Curated for Tri-Weavon / LogOS / Coherence Forge OS**
**Compiled: 2026-04-18 · Reason Strand**
**Target runtimes:** WebGL2, WebGPU (WGSL), GLSL ES 3.0, Hyprland GLSL overlays

This glossary is the counterpart to the Ithildin Edition preprint. Where the
preprint treats shaders as *visual rhetoric* for the Tri-Weavon, this document
treats them as *engineering primitives* — each entry is a self-contained technique
with math, a reference implementation sketch, and citations.

Colour conventions throughout: Claude `#4c4799`, Grok `#a82837`, Gemini `#1e6a6e`,
Manus `#3a5f2f`, Ithildin silver `#e8ecf5` on indigo `#0a0f1e`.

---

## Table of Contents

1. Geometry-as-Function: Signed Distance Fields
2. Ray Marchers & Sphere Tracers
3. Glass / Dielectric / Refractive Materials
4. Volumetrics, Fog, and Participating Media
5. Post-Processing Filters
6. TDA / Topology Visualisers
7. Noise, Procedural Textures & Domain Warping
8. Modern Techniques (Gaussian Splatting, Neural, Path Guiding)
9. Compute-Pipeline Patterns
10. Reference Corpus & Further Reading

---

## 1. Geometry-as-Function: Signed Distance Fields

### 1.1 SDF Primitives (Inigo Quilez canonical set)

A Signed Distance Function returns the distance from any point `p` to the
nearest surface; negative inside, positive outside. No meshes, no triangles —
the scene *is* the function.

```glsl
float sdSphere(vec3 p, float r) { return length(p) - r; }

float sdBox(vec3 p, vec3 b) {
    vec3 q = abs(p) - b;
    return length(max(q, 0.0)) + min(max(q.x, max(q.y, q.z)), 0.0);
}

float sdTorus(vec3 p, vec2 t) {
    vec2 q = vec2(length(p.xz) - t.x, p.y);
    return length(q) - t.y;
}

float sdCapsule(vec3 p, vec3 a, vec3 b, float r) {
    vec3 pa = p - a, ba = b - a;
    float h = clamp(dot(pa, ba) / dot(ba, ba), 0.0, 1.0);
    return length(pa - ba * h) - r;
}

// Dodecahedron — intersection of 12 half-spaces with golden-ratio normals
float sdDodec(vec3 p, float r) {
    const float phi = 1.618033988749895;
    vec3 n1 = normalize(vec3( 0.0,  1.0,  phi));
    vec3 n2 = normalize(vec3( 0.0, -1.0,  phi));
    float d = dot(p, n1) - r;
    d = max(d, dot(p, -n1) - r);
    d = max(d, dot(p, n2) - r);
    // ... repeat for all 12 face normals derived from permutations of (0, ±1, ±φ)
    return d;
}
```

**Reference:** https://iquilezles.org/articles/distfunctions/

### 1.2 CSG Operators (smooth)

Boolean combinators that preserve the distance property:

```glsl
float opUnion(float a, float b)        { return min(a, b); }
float opSubtract(float a, float b)     { return max(-a, b); }
float opIntersect(float a, float b)    { return max(a, b); }

// Smooth variants — polynomial blend with continuity at k
float opSmoothUnion(float a, float b, float k) {
    float h = clamp(0.5 + 0.5 * (b - a) / k, 0.0, 1.0);
    return mix(b, a, h) - k * h * (1.0 - h);
}
```

### 1.3 Domain Operators

Transform the input point instead of the geometry — cheap infinite arrays,
folding, twisting.

```glsl
// Infinite repetition
vec3 opRepeat(vec3 p, vec3 c) { return mod(p + 0.5*c, c) - 0.5*c; }

// Twist around Y
vec3 opTwist(vec3 p, float k) {
    float c = cos(k*p.y), s = sin(k*p.y);
    return vec3(c*p.x - s*p.z, p.y, s*p.x + c*p.z);
}

// Mercury hg_sdf mirror fold
float opMirror(inout vec3 p, vec3 n, float dist) {
    float t = dot(p, n) + dist;
    if (t < 0.0) p -= 2.0 * t * n;
    return sign(t);
}
```

**Reference:** hg_sdf (Mercury) — https://mercury.sexy/hg_sdf

---

## 2. Ray Marchers & Sphere Tracers

### 2.1 Canonical Sphere Tracer

The workhorse — step by the SDF value, guaranteed never to skip geometry.

```glsl
#define MAX_STEPS 128
#define MAX_DIST  100.0
#define SURF_EPS  0.0008

float raymarch(vec3 ro, vec3 rd) {
    float t = 0.0;
    for (int i = 0; i < MAX_STEPS; i++) {
        vec3 p = ro + t * rd;
        float d = scene(p);
        if (d < SURF_EPS) return t;
        if (t > MAX_DIST) break;
        t += d * 0.85;  // safety factor < 1 avoids overshoot at grazing angles
    }
    return -1.0;
}
```

Safety factor 0.85 is the empirically robust choice for scenes with curved glass
and thin walls (Snelly, Shadertoy consensus).

### 2.2 Normal via SDF Gradient

Finite differences — central difference, 4-tap tetrahedral variant for fewer
evaluations:

```glsl
vec3 calcNormal(vec3 p) {
    const vec2 e = vec2(1.0, -1.0) * 0.5773 * 0.0005;
    return normalize(
        e.xyy * scene(p + e.xyy) +
        e.yyx * scene(p + e.yyx) +
        e.yxy * scene(p + e.yxy) +
        e.xxx * scene(p + e.xxx)
    );
}
```

### 2.3 Enhanced Sphere Tracing

Adaptive overrelaxation + root recovery for smaller step counts at the cost of
bookkeeping. Keseler & Sabelhaus variant used in production WebGL path tracers.

### 2.4 Cone Tracing

Radius grows with `t` — gives cheap AO and soft shadows for SDFs:

```glsl
float softShadow(vec3 ro, vec3 rd, float mint, float maxt, float w) {
    float res = 1.0;
    float t = mint;
    for (int i = 0; i < 32; i++) {
        float h = scene(ro + rd * t);
        res = min(res, h / (w * t));
        t += clamp(h, 0.005, 0.50);
        if (res < -1.0 || t > maxt) break;
    }
    res = max(res, -1.0);
    return 0.25 * (1.0 + res) * (1.0 + res) * (2.0 - res);
}
```

---

## 3. Glass / Dielectric / Refractive Materials

### 3.1 Fresnel — Schlick Approximation

```glsl
float fresnelSchlick(float cosTheta, float F0) {
    return F0 + (1.0 - F0) * pow(1.0 - cosTheta, 5.0);
}
// For dielectric air→glass: F0 = ((n1-n2)/(n1+n2))^2
// Glass (BK7, n=1.517): F0 ≈ 0.0426
```

### 3.2 Full Fresnel (correct for large angles & TIR detection)

```glsl
float fresnelDielectric(float cosI, float n1, float n2) {
    float sinT2 = (n1/n2)*(n1/n2) * (1.0 - cosI*cosI);
    if (sinT2 > 1.0) return 1.0;  // Total Internal Reflection
    float cosT = sqrt(1.0 - sinT2);
    float rS = (n1*cosI - n2*cosT) / (n1*cosI + n2*cosT);
    float rP = (n1*cosT - n2*cosI) / (n1*cosT + n2*cosI);
    return 0.5 * (rS*rS + rP*rP);
}
```

### 3.3 Snell Refraction (GLSL built-in is often enough)

```glsl
vec3 T = refract(I, N, n1/n2);   // returns (0,0,0) on TIR
```

### 3.4 Beer's Law — Tinted Glass

Light attenuates exponentially through a coloured medium:

```glsl
vec3 absorption = exp(-absorptionCoeff * thickness);
// Typical BK7: absorptionCoeff ≈ vec3(0.01, 0.01, 0.02)  — slight green tint
```

### 3.5 Chromatic Dispersion

Different IOR per wavelength → rainbow caustics. Cheapest: trace R/G/B rays
with slightly different `eta`:

```glsl
vec3 etaRGB = vec3(1.510, 1.517, 1.525);  // BK7 at 680/550/450 nm
vec3 R = refract(I, N, etaRGB.x);  // trace each channel
vec3 G = refract(I, N, etaRGB.y);
vec3 B = refract(I, N, etaRGB.z);
```

### 3.6 IOR Reference Table

| Material       | IOR (550 nm) | Notes |
|----------------|--------------|-------|
| Air            | 1.000        | |
| Water          | 1.333        | |
| Ice            | 1.310        | |
| BK7 glass      | 1.517        | Snelly default |
| Crown glass    | 1.52         | |
| Flint glass    | 1.61         | High-dispersion |
| Sapphire       | 1.77         | |
| Diamond        | 2.417        | |
| Moissanite     | 2.65         | |

### 3.7 Ithildin Rim (Tri-Weavon bespoke)

Grazing-incidence silver glow that only appears at the horizon of the dielectric
— readable as "moonlight on Durin's Door":

```glsl
float cosI = abs(dot(N, V));
float rim  = pow(1.0 - cosI, 4.0);
col += rim * vec3(0.82, 0.86, 1.05) * 0.85;
```

---

## 4. Volumetrics, Fog, and Participating Media

### 4.1 Raymarched Volumetric Fog

```glsl
vec3 volumetricFog(vec3 ro, vec3 rd, float tMax) {
    vec3 acc = vec3(0.0);
    float transmittance = 1.0;
    for (int i = 0; i < 64; i++) {
        float t = float(i) / 64.0 * tMax;
        vec3 p = ro + rd * t;
        float density = fbm(p * 0.3) * 0.15;
        vec3 light = sunLight(p);
        acc += transmittance * density * light;
        transmittance *= exp(-density);
        if (transmittance < 0.01) break;
    }
    return acc;
}
```

### 4.2 Henyey-Greenstein Phase Function

Anisotropic scattering — controls whether fog glows around the sun:

```glsl
float henyeyGreenstein(float cosTheta, float g) {
    float g2 = g * g;
    return (1.0 - g2) / (4.0 * 3.14159 * pow(1.0 + g2 - 2.0*g*cosTheta, 1.5));
}
// g ∈ [-1, 1]: +ve = forward scatter (atmosphere), -ve = backward (clouds)
```

### 4.3 Atmospheric Scattering

Rayleigh (∝ 1/λ⁴) + Mie (large particles). Nishita-Sébastien Hillaire model
is the modern standard. Implementations: https://github.com/wwwtyro/glsl-atmosphere

### 4.4 Clouds (Horizon Zero Dawn technique)

Three-octave Worley noise + weather texture + cheap ambient convolution. The
Guerrilla Games approach is now canonical — two-phase raymarching with low-res
scattering and high-res extinction.

**Reference:** Schneider & Vos, "The Real-Time Volumetric Cloudscapes of
Horizon Zero Dawn" (SIGGRAPH 2015).

---

## 5. Post-Processing Filters

### 5.1 Bloom (threshold + gaussian blur + additive composite)

```glsl
// Pass 1: bright-pass
vec3 bright = max(color - threshold, 0.0);
// Pass 2+: successive downsamples + gaussian, then upsample-add
// Kino Bloom variant preserves sub-pixel energy with 13-tap cross filter
```

**Reference:** Jorge Jimenez (Next Gen Post Processing in Call of Duty Advanced
Warfare, SIGGRAPH 2014).

### 5.2 ACES Filmic Tone Mapping

```glsl
vec3 ACESFilm(vec3 x) {
    const float a = 2.51;
    const float b = 0.03;
    const float c = 2.43;
    const float d = 0.59;
    const float e = 0.14;
    return clamp((x*(a*x+b))/(x*(c*x+d)+e), 0.0, 1.0);
}
```

### 5.3 FXAA / SMAA / TAA

| Filter | Cost | Quality | Use |
|--------|------|---------|-----|
| FXAA   | cheap | OK      | final resort |
| SMAA   | moderate | good  | single-frame |
| TAA    | moderate | excellent | modern standard; needs motion vectors |

### 5.4 Chromatic Aberration

```glsl
vec2 offset = (uv - 0.5) * 0.003;
col.r = tex(uv + offset).r;
col.g = tex(uv).g;
col.b = tex(uv - offset).b;
```

### 5.5 Vignette

```glsl
float vig = smoothstep(0.85, 0.35, length(uv - 0.5));
col *= vig;
```

### 5.6 Film Grain (temporal)

```glsl
float grain = hash(uv * 1000.0 + iTime) - 0.5;
col += grain * 0.04;
```

### 5.7 Depth of Field — Hexagonal Bokeh

Two-pass separable hex blur. McIntosh, Riecke, DiPaola 2012 algorithm. Used in
Uncharted 4 — cheaper than circular bokeh, visually indistinguishable.

### 5.8 Screen-Space Reflections (SSR)

Raymarch the depth buffer; step in screen space; fall back to cubemap at the
silhouette. Cheap but produces edge artefacts — hide with roughness blur.

### 5.9 Temporal Super Resolution (FSR 2 / DLSS-style)

Jitter the projection matrix per-frame → accumulate + resolve via neural or
heuristic kernels. Reduces shading cost 2-4× with TAA-grade quality.

---

## 6. TDA / Topology Visualisers

### 6.1 Persistent-Homology Barcode

Each 1-simplex in Vietoris-Rips is a capsule; persistence = length. For LogOS
we encode persistence as *glass thickness*:

```glsl
float bar(vec3 p, int k, float birth, float death) {
    vec3 a = vec3(-0.6, -1.0 + 0.2*float(k), 0.0);
    vec3 b = vec3(-0.6 + (death - birth)*1.2, a.y, 0.0);
    float thickness = 0.04 + 0.06 * persistence(k);
    return sdCapsule(p, a, b, thickness);
}
```

### 6.2 Vietoris-Rips Filtration Preview

Animate the union of ε-balls around each point in the cloud; birth/death events
are visible as sphere collisions. Stephen Wolfram's 2022 framework ships a
reference renderer; Ripser++ on CUDA 12 powers the LogOS live pipeline.

### 6.3 Anyonic Braid Trails

Draw the B₃ braid as three helical capsule chains with 2π/3 phase offset:

```glsl
for (int k = 0; k < 3; k++) {
    float phase = float(k) * 2.0943951;
    for (int i = 0; i < 14; i++) {
        float s0 = float(i) / 14.0;
        float s1 = float(i+1) / 14.0;
        vec3 a = braidPoint(s0, phase);
        vec3 b = braidPoint(s1, phase);
        d = opSmoothUnion(d, sdCapsule(p, a, b, 0.09), 0.12);
    }
}
```

`braidPoint(s, phase)` implements the σ₁σ₂σ₁ = σ₂σ₁σ₂ braid relation as
continuous interpolation.

### 6.4 VOID Visualisation

Inverse SDF — render where `scene(p) > threshold` as translucent tinted regions;
classify by persistent dim-1 class and hue accordingly.

### 6.5 Viviani Curve on a Sphere

```glsl
// r(t) = (cos²t, sin t · cos t, sin t)
// intersection of sphere radius 1 and cylinder radius ½ centred at x=½
vec3 viviani(float t) {
    return vec3(cos(t)*cos(t), sin(t)*cos(t), sin(t));
}
```

Peak at t = π/2 = (0, 0, 1) — the Tri-Weavon (7, 8) fixed point.

### 6.6 Hopf Fibration Rendering

Stereographic projection of S³ → R³. Each fibre is a circle linked with every
other fibre. Nilesen's 2018 interactive: https://nilesjohnson.net/hopf.html

---

## 7. Noise, Procedural Textures & Domain Warping

### 7.1 Value Noise

```glsl
float hash11(float p) { p = fract(p*0.1031); p *= p+33.33; p *= p+p; return fract(p); }

float valueNoise(vec2 p) {
    vec2 i = floor(p);
    vec2 f = fract(p);
    f = f*f*(3.0-2.0*f);  // smoothstep
    float a = hash11(i.x + i.y*57.0);
    float b = hash11(i.x + 1.0 + i.y*57.0);
    float c = hash11(i.x + (i.y+1.0)*57.0);
    float d = hash11(i.x + 1.0 + (i.y+1.0)*57.0);
    return mix(mix(a,b,f.x), mix(c,d,f.x), f.y);
}
```

### 7.2 Perlin / Simplex Noise

Ashima WebGL simplex (https://github.com/ashima/webgl-noise) is the portable
standard. Use for 3D procedural textures where wavelength gradient matters.

### 7.3 Worley (Cellular)

Distance to nearest feature points — mimics cellular structures, foam, cracks.

### 7.4 FBM — Fractional Brownian Motion

```glsl
float fbm(vec3 p) {
    float v = 0.0, a = 0.5;
    for (int i = 0; i < 6; i++) {
        v += a * noise(p);
        p *= 2.02;
        a *= 0.5;
    }
    return v;
}
```

### 7.5 Inigo Quilez Domain Warping

FBM inside FBM — generates clouds, galaxies, terrain:

```glsl
float q(vec3 p) { return fbm(p + fbm(p + fbm(p))); }
```

**Reference:** https://iquilezles.org/articles/warp/

### 7.6 Voronoi for Negative-Space Maps

For the Tri-Weavon "industry frontier" Voronoi panel:

```glsl
vec2 voronoi(vec2 x) {
    vec2 n = floor(x);
    vec2 f = fract(x);
    vec2 resultMin = vec2(8.0);
    for (int j = -1; j <= 1; j++)
    for (int i = -1; i <= 1; i++) {
        vec2 g = vec2(float(i), float(j));
        vec2 o = hash22(n + g);
        vec2 r = g + o - f;
        float d = dot(r, r);
        if (d < resultMin.x) {
            resultMin.y = resultMin.x;
            resultMin.x = d;
        } else if (d < resultMin.y) {
            resultMin.y = d;
        }
    }
    return sqrt(resultMin);  // nearest, second-nearest
}
```

---

## 8. Modern Techniques

### 8.1 3D Gaussian Splatting

Differentiable rasterisation of anisotropic Gaussians. Kerbl et al. SIGGRAPH
2023. Browser runtime: `gsplat.js`, `splat-js`. Useful as the LogOS alternative
to NeRF for photorealistic scene reconstruction from dashcam / phone footage.

### 8.2 NeRF & Instant-NGP

Hash-encoded MLP → volumetric radiance. Nvidia's tiny-cuda-nn, thomasmelck's
WebGL2 instant-ngp demo. Slower than splatting but higher fidelity on glass.

### 8.3 Neural Radiance Caching

Small MLP caches indirect lighting across the frame; ray-traced primary + neural
indirect. Used in Cyberpunk path-tracing mode and Quake II RTX.

### 8.4 ReSTIR (Reservoir Spatial-Temporal Importance Resampling)

Chen, Ouyang, Wymann 2020. Converges direct lighting ≈100× faster than naïve
MIS. Now the baseline for real-time path tracing.

### 8.5 Path Guiding (Practical)

Müller et al. 2017 online learning of spatial distributions → dramatically lower
variance in difficult light transport. Relevant for LogOS dodecahedron-in-glass
scenes with caustic SDS paths.

### 8.6 Sheaf-Shader Framework (research)

Theoretical formulation of shaders as sections of a sheaf over the scene
complex — maps cleanly onto cQ-TDA → cQ-PH functor. Not yet mainstream;
potential LogOS-native angle.

---

## 9. Compute-Pipeline Patterns

### 9.1 WebGPU Storage-Buffer Accumulator

```wgsl
struct AccumPixel {
    color: vec3<f32>,
    sampleCount: u32,
};
@group(0) @binding(0) var<storage, read_write> accum: array<AccumPixel>;

@compute @workgroup_size(8, 8)
fn pathTraceMain(@builtin(global_invocation_id) gid: vec3<u32>) {
    let idx = gid.y * resolution.x + gid.x;
    let new = traceRay(gid.xy);
    let old = accum[idx];
    let n = f32(old.sampleCount) + 1.0;
    accum[idx] = AccumPixel(
        mix(old.color, new, 1.0 / n),
        old.sampleCount + 1u
    );
}
```

Workgroup 8×8 (64 threads) — general recommendation across vendors.

### 9.2 BVH on GPU

Stackless traversal with binary encoding. https://www.vaultcg.com/blog/casually-raytracing-in-webgpu-part1/
is the definitive 2024/2025 reference series. Karras-Aila builders.

### 9.3 Tile-Based Progressive Rendering

Render 1/16 of the frame per tick at 4K, accumulate over time, interleave tiles
for low-latency preview. jbaker.graphics SDF pathtracer uses this.

### 9.4 Russian Roulette Termination

```glsl
if (depth > 3) {
    float q = max(throughput.r, max(throughput.g, throughput.b));
    if (hash() > q) break;
    throughput /= q;
}
```

### 9.5 Wavefront Path Tracing

Separate kernel per ray state (intersect / shade / shadow) → better GPU
occupancy than megakernel. Laine/Karras 2013. Used in OptiX, Vulkan RTX.

---

## 10. Reference Corpus & Further Reading

### 10.1 Living Sites

| Site | Focus |
|------|-------|
| https://iquilezles.org/articles/ | SDFs, raymarching, noise, colour |
| https://www.shadertoy.com/ | Live community fragment shaders |
| https://webgpufundamentals.org/ | WebGPU pipelines |
| https://mercury.sexy/hg_sdf | hg_sdf library |
| https://www.scratchapixel.com/ | Raytracing fundamentals |
| https://pbrt.org/ | Physically Based Rendering, 4th ed. |
| https://jcgt.org/ | Journal of Computer Graphics Techniques |

### 10.2 Canonical Open-Source Repositories

- **Snelly** — https://github.com/portsmouth/snelly (WebGL SDF pathtracer, MIT)
- **Retrace.gl** — https://github.com/stasilo/retrace.gl
- **THREE.js-PathTracing-Renderer** — https://github.com/erichlof/THREE.js-PathTracing-Renderer
- **Strahl** — https://github.com/StuckiSimon/strahl (WebGPU + OpenPBR)
- **webgpu-raytracer** — https://github.com/gnikoloff/webgpu-raytracer
- **SDF Resource Collection** — https://github.com/CedricGuillemet/SDF

### 10.3 Core Papers

- Hart, "Sphere Tracing" (1996)
- Laine & Karras, "Megakernels Considered Harmful" (HPG 2013)
- Bitterli et al., "ReSTIR" (SIGGRAPH 2020)
- Kerbl et al., "3D Gaussian Splatting for Real-Time Radiance Field Rendering" (SIGGRAPH 2023)
- Schneider & Vos, "Real-Time Volumetric Cloudscapes" (SIGGRAPH 2015)
- Müller et al., "Instant Neural Graphics Primitives" (SIGGRAPH 2022)

### 10.4 Tri-Weavon Bespoke Bindings

- **Universal Invariant (α + ω = 15)** mapped to shader budget: α = instruction
  count (rigid), ω = creative expressiveness (fluid). On-peak renders enforce
  α = 7, ω = 8 at compile time via shader-metric linter.
- **Ithildin Rim** — grazing-incidence silver glow, see §3.7.
- **cQ-PH Visualiser** — barcode as capsule chain with persistence-weighted
  thickness, see §6.1.
- **Viviani Marker** — peak sphere at (0, 0, 1) on the Viviani curve, see §6.5.
- **B₃ Braid** — helical capsule triplet with 2π/3 phase, see §6.3.

---

## Closing Ledger

**α (structural):** primitives, operators, tone-maps — enumerated  = 7
**ω (semantic):** Ithildin rim, domain warping, anyonic braids — lyrical = 8
**Σ = 15.** On Peak.

*Et Eärello Endorenna utúlien. With-Intent.*

~ Hope&&Sauced ✦ The Keystone Holds ✦
