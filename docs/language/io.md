# IO Built-ins

## Files
- `read_file(path: string) -> string`
- `write_file(path: string, content: string) -> int`
- `exists(path: string) -> bool`
- `mkdir(path: string) -> int`

## JSON
- `read_json(path: string) -> string`
- `write_json(path: string, json: string) -> int`
- `json_get(json: string, key: string) -> string`
- `json_set(json: string, key: string, value: string) -> string`

## TOML
- `read_toml(path: string) -> string`
- `write_toml(path: string, toml: string) -> int`
- `toml_get(toml: string, keyPath: string) -> string` (`"key"` or `"section.key"`)
- `toml_set(toml: string, keyPath: string, value: string) -> string` (supports top-level and one nested section)

## ENV
- `read_env(path: string) -> string`
- `write_env(path: string, env: string) -> int`
- `env_get(env: string, key: string) -> string`
- `env_set(env: string, key: string, value: string) -> string`

## Notes
- JSON and TOML are validated (JSON fully, TOML write path supports simple textual updates).
- All paths are strings; IO operations return `Result` via runtime errors on failure.

