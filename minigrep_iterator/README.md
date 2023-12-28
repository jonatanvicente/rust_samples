

### minigrep_iterator

- Mejora del proyecto minigrep_env
- Carga de parámetros recibidos por línea de comandos con iterator
- Utilización de iterators en métodos de búsqueda search y search_case_insensitive
- **OJO**: en Rust, la utilización de Iterators tiene mejor performance que emplear loops (for, de más bajo nivel).