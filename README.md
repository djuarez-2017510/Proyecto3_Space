# Space Travel - Sistema Solar en Rust

Proyecto de gráficas 3D que renderiza un sistema solar con planetas en órbita usando Rust puro.

## VIDEO

[![Ver video](https://img.youtube.com/vi/mHz13Y6t_ig/0.jpg)](https://youtu.be/mHz13Y6t_ig)


## Compilación y Ejecución

```bash
cargo build --release
cargo run --release
```


## Estructura del Proyecto

```
proyecto3_space/
├── src/
│   ├── main.rs              - Loop principal, warping, órbitas, skybox
│   ├── framebuffer.rs       - Framebuffer y z-buffer
│   ├── vertex.rs            - Estructura de vértice
│   ├── shaders.rs           - Shaders y matrices
│   ├── triangle.rs          - Rasterización
│   ├── obj.rs               - Cargador de OBJ
│   ├── camera.rs            - Sistema de cámara
│   ├── planet_shaders.rs    - Shaders de planetas
│   └── math.rs              - Matemáticas 3D desde cero
├── assets/
│   └── sphere.obj           - Modelo de esfera
└── Cargo.toml
```





## Dependencias

- `minifb`: Ventana 

