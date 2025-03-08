# **JAM Whitelist Authorizer**

Este proyecto implementa un **autorizador basado en roles y whitelist** utilizando el SDK de JAM (Parity). Permite gestionar direcciones autorizadas, roles de usuarios (administrador, participante, observador) y un juego interactivo donde los participantes responden preguntas. Los resultados son supervisados por observadores y administradores.

---

## **Tabla de Contenidos**

1. [Descripción del Proyecto](#descripción-del-proyecto)
2. [Componentes del SDK de JAM](#componentes-del-sdk-de-jam)
3. [Requisitos del Sistema](#requisitos-del-sistema)
4. [Instalación](#instalación)
5. [Ejecución del Proyecto](#ejecución-del-proyecto)
6. [Estructura del Proyecto](#estructura-del-proyecto)
7. [Personalización](#personalización)
8. [Contribuciones](#contribuciones)
9. [Licencia](#licencia)

---

## **Descripción del Proyecto**

Este programa utiliza el SDK de JAM para implementar un sistema de autorización basado en roles y whitelist. Incluye las siguientes funcionalidades:

- **Roles**: Administrador, participante y observador.
- **Whitelist**: Solo las direcciones autorizadas pueden realizar ciertas acciones.
- **Juego interactivo**: El participante responde preguntas definidas por el administrador.
- **Resultados**: Los resultados del juego son supervisados por el observador o administrador.

El proyecto está diseñado para ser modular y fácil de extender, permitiendo agregar nuevas preguntas, roles o funcionalidades.

---

## **Componentes del SDK de JAM**

El SDK de JAM proporciona varias herramientas y estructuras clave que se utilizan en este proyecto. A continuación, se describe cada componente y cómo se integra en el código:

### **1. `jam-types`**
- **Descripción**: Este crate contiene tipos básicos y estructuras necesarias para trabajar con servicios y autorizadores en JAM.
- **Componentes utilizados**:
  - **`Authorizer`**: Representa el autorizador del sistema JAM. Contiene dos campos principales:
    - `code_hash`: Un hash que representa el código del programa.
    - `param`: Un parámetro que puede almacenar información adicional (por ejemplo, la dirección del administrador).
  - **`CodeHash`**: Representa un hash de 32 bytes utilizado para validar si un hash pertenece a una lista de hashes válidos.
  - **`AuthParam`**: Un parámetro que se utiliza para almacenar datos adicionales relacionados con la autorización.
- **Uso en el proyecto**:
  - Se utiliza `Authorizer` para gestionar la validación de direcciones autorizadas.
  - Se genera dinámicamente un `CodeHash` basado en el nombre del administrador y se utiliza para verificar la autenticidad de las operaciones.
  - Se extiende `AuthParam` con la dirección del administrador para rastrear quién realizó ciertas acciones.

### **2. `jam-pvm-common`**
- **Descripción**: Proporciona funcionalidades comunes para trabajar con el PVM (Parity Virtual Machine).
- **Componentes utilizados**:
  - **Funciones de serialización/deserialización**: Utilizadas para manejar datos en el formato requerido por el PVM.
  - **Estructuras de datos compartidas**: Facilitan la interoperabilidad entre diferentes módulos del SDK.
- **Uso en el proyecto**:
  - Se utiliza para asegurar que los datos generados por el autorizador sean compatibles con el entorno de ejecución del PVM.

### **3. `jam-bootstrap-service`**
- **Descripción**: Un servicio básico útil para crear configuraciones iniciales (genesis) en JAM.
- **Uso en el proyecto**:
  - Se clona como referencia para comprender cómo configurar un servicio básico en JAM.

### **4. `jam-null-authorizer`**
- **Descripción**: Un autorizador básico que permite realizar pruebas sin restricciones de autorización.
- **Uso en el proyecto**:
  - Se utiliza como punto de partida para implementar el autorizador personalizado (`RoleBasedWhitelistAuthorizer`).

### **5. `jam-pvm-build`**
- **Descripción**: Una herramienta CLI que permite construir blobs de código PVM para servicios o autorizadores.
- **Uso en el proyecto**:
  - Se utiliza para compilar el autorizador personalizado en un blob PVM que pueda ser desplegado en el entorno de JAM.

---

## **Requisitos del Sistema**

Para ejecutar este proyecto, necesitas lo siguiente:

1. **Sistema operativo**:
   - Linux, macOS o Windows (con soporte para herramientas de línea de comandos como Git Bash o PowerShell).

2. **Herramientas requeridas**:
   - [Git](https://git-scm.com/)
   - [Rust](https://www.rust-lang.org/) (versión `nightly-2024-11-01` o superior)
   - Cargo (gestor de paquetes de Rust, instalado automáticamente con Rust)

3. **SDK de JAM**:
   - Dependencias específicas del SDK (`jam-types`, `jam-pvm-common`, etc.).

---

## **Instalación**

Sigue estos pasos para configurar el entorno y descargar las dependencias necesarias:

### **1. Instalar Rust**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env