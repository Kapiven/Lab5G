# Sistema Solar 3D en Rust ☀️🪐🌍🌑

Este proyecto implementa una simulación sencilla de un sistema solar en **Rust** utilizando renderizado en 3D.  
Incluye una **estrella**, un **planeta rocoso**, un **planeta gaseoso con anillos** y una **luna orbitando** alrededor del planeta rocoso.

El sistema presenta:
- Movimiento orbital dinámico basado en tiempo
- Distancias ajustadas entre cuerpos
- Anillos en el planeta gaseoso
- Efecto de brillo en la estrella
- Rotación continua del sistema
- Vistas en perspectiva para observar el movimiento

---

## Vista Previa de la Simulación

![Vista previa](./out/Video-Espacio.gif)

---

## Cuerpos del Sistema

| Cuerpo Celeste      | Descripción | Imagen |
|--------------------|-------------|--------|
| ⭐ **Estrella**     | Fuente principal de luz; color amarillo brillante con halo. | ![Star](./out/star.png) |
| 🌍 **Planeta Rocoso** | Pequeño, con superficie sólida. Tiene una luna orbitándolo. | ![Rocky Planet](./out/rocoso.jpg) |
| 🌕 **Luna**         | Órbita corta alrededor del planeta rocoso. | ![Moon](./out/rocoso.jpg) |
| 🪐 **Planeta Gaseoso** | Gigante con coloración suave y **anillos visibles**. | ![Gas Giant](./out/gaseoso.jpg) |

---

## Tecnologías Utilizadas

| Tecnología | Uso |
|-----------|-----|
| **Rust** | Lógica del simulador y renderizado |
| **raylib / minifb / glium (dependiendo de tu código)** | Renderizado y ventana |
| **Vec3 / Ray / Sphere** (implementaciones propias) | Matemática de escenas 3D |

---

## Instalación y Ejecución

### 1. Clonar el repositorio
```bash
git clone https://github.com/Kapiven/Lab5G.git

```

## Autor

Karen Pineda :]
