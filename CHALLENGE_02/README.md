# Antes de ejecutar
Cuando ejecutes este programa, asegúrate de pasar el input entre comillas. Esto es necesario porque algunos símbolos (como &) pueden ser interpretados por tu shell como comandos para iniciar un nuevo proceso en el fondo. Encerrar el input entre comillas asegura que toda la cadena de caracteres se pase al programa como un único argumento.

```rs
cargo run "&###@&*&###@"
```

## AVISO
**Importante**: Si no encierras el input entre comillas, es posible que el programa no se ejecute correctamente y que la shell inicie procesos inesperados en el fondo.