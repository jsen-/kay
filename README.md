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
```
$ cat dashboard-user.yaml
apiVersion: v1
kind: ServiceAccount
metadata:
  name: ${env KUBE_USER}
  namespace: ${env NAMESPACE}

$ NAMESPACE=test-ns KUBE_USER=karol kay --input-file=dashboard-user.yaml
apiVersion: v1
kind: ServiceAccount
metadata:
  name: karol
  namespace: test-ns
```

## TODO
 - respect BOM
 - `--vars-file` ... well, yes, this is not yet implemented
