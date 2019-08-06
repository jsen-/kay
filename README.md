# kay
replace `${...}` expressions in text

## description
```
USAGE:
    kay [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input-file <input-file>      
    -o, --output-file <output-file>    
    -v, --vars-file <vars-file>        
        --vars-format <vars-format>    
```

## usage
 - if `--input-file` / `-i` is omited, input will be expected from `stdin`
 - if `--output-file` / `-o` is omited, output will go to `stdout`
 - if `--vars-file` is omited, input may only use environment variables
 - `\${ ... }` will *not* be translated
 - format of `vars file` file is inferred from extension `.yaml`/`.yml`/`.json` but can be specified by `--vars-format yaml` or `--vars-format json`
 - input must be utf8 (this might change, see TODO)

## example
`input.txt`
```
hello ${var $.world}!
```
`vars.json`:
```json
{ "world": "jsonWORLD!" }
```
`vars.yaml`:
```yaml
world: yamlWORLD
```
```bash
$ kay -i input.txt --vars-file vars.json 
hello jsonWORLD
```
```bash
$ kay -i input.txt --vars-file vars.yaml 
hello yamlWORLD
```
```bash
$ echo 'happy ${env TEST_VAR}!' | TEST_VAR="birthday" kay
happy birthday!
```

## TODO
 - respect BOM
 - make a proper expression parser
 - support for piping expression results
