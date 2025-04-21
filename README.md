# Galar 🌀  
**Motor Gráfico Experimental en Rust**

> Un motor gráfico 2D minimalista y experimental. Utiliza `minifb` para renderizado en framebuffer y se enfoca en la exploración de conceptos como generación procedural, optimización matemática, y sistemas de renderizado personalizados.

---

## ✨ Características

- 🔧 **Sistema de Plugins** extensible mediante `trait PluginGalar`.
- 🖥️ **Renderizado rápido** en `minifb` con control total del framebuffer.
- 🧠 **Algoritmos matemáticos personalizados** (e.g. `fast_sqrt`, `remap`, `proximidad`)
- 🎨 Sistema de color completo (`Color`): RGB, HSL, aleatorio, interpolado, gradientes térmicos y más.
- 🔺 **Render de triángulos** con transformación, color y textura.
- 📦 Sistema de configuración flexible (`ConfigGalar`)
- 🧼 ~Limpieza automática de buffer por ciclo de renderizado.~
- 🧭 ~Gestión básica de eventos y entradas del usuario.~

---

## ⚠️ Estado del Proyecto

> **Este proyecto es altamente experimental.**  
> La API, funciones internas y estructuras pueden cambiar sin previo aviso.  
> Úsalo bajo tu propio riesgo y con espíritu explorador. No se recomienda para producción (todavía).
