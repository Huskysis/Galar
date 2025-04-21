# Galar 🌀

**Galar** es un motor gráfico 2D minimalista y experimental escrito en **Rust**, orientado a la creación de sistemas personalizados y exploración de mecánicas gráficas no convencionales.

Este motor está pensado como una **herramienta de investigación y aprendizaje**, ideal para quienes deseen construir su propio stack visual desde cero, enfocándose en conceptos como renderizado personalizado, sistemas de color avanzados, matemáticas optimizadas y estructuras modulares por plugins.

## ⚙️ Características Actuales

- Renderizado en ventana mediante [`minifb`](https://github.com/emoon/rust_minifb).
- Utilidades matemáticas optimizadas (`fast_sqrt`, remapeo, interpolaciones).
- Sistema de color completo (`Color`): RGB, HSL, aleatorio, interpolado, gradientes térmicos y más.
- Geometría y proximidad en espacio 2D.
- Soporte para extensibilidad vía traits (`PluginGalar`).

## ⚠️ Estado del Proyecto

> **Este proyecto es altamente experimental.**  
> La API, funciones internas y estructuras pueden cambiar sin previo aviso.  
> Úsalo bajo tu propio riesgo y con espíritu explorador. No se recomienda para producción (todavía).
