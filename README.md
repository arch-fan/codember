# CODEMBER

Este repositorio contiene las soluciones a los desafíos del [Codember](https://codember.dev/) de midudev, implementadas en el lenguaje de programación Rust.

~~Cada desafío se presenta como un crate independiente dentro de este workspace, permitiendo una organización clara y una ejecución fácil de cada solución.~~

![Ferris, la mascota de Rust](https://rustacean.net/assets/cuddlyferris.png)

## Ejecución de los Ejercicios

Si ejecutas el proyecto principal, se ejecutarán todos los ejercicios de manera secuencial. Para hacer esto, sigue los siguientes pasos:

1. Asegúrate de tener [Rust y Cargo instalados](https://www.rust-lang.org/tools/install) en tu sistema.
2. Clona el repositorio a tu máquina local usando Git:
   
   ```bash
   git clone <url-del-repositorio>
   cd codember
   ```
3. Accede al ejercicio deseado y ejecuta el codigo (importante leer el README dentro de cada challenge)

    ```bash
    cd CHALLENGE_0X
    cargo run
    ```

<!-- 3. Para ejecutar todos los ejercicios a la vez, utiliza el siguiente comando en la terminal:

   ```bash
   cargo run
   ```

   Este comando compilará y ejecutará el crate principal, que a su vez ejecutará cada ejercicio en el orden definido. -->

## Estructura del Proyecto

Cada desafío está organizado en su propio subdirectorio y puede ser ejecutado individualmente. La estructura del proyecto es la siguiente:

- `CHALLENGE_01`: Contiene la solución al primer desafío.
- `CHALLENGE_02`: Contiene la solución al segundo desafío.
- ... y así sucesivamente para los demás desafíos.

## Contribuciones y Mejoras

Las contribuciones al código son bienvenidas. Si tienes sugerencias o mejoras, no dudes en abrir un pull request o una issue.

## Licencia

Este proyecto está bajo una licencia [MIT](https://opensource.org/licenses/MIT), lo que significa que puedes usarlo libremente para tus propios proyectos y aprendizaje.