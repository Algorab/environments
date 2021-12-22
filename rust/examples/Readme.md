# Examples

## Platforms
Currently only supported:
* linux/amd64

## fission-rust-handler
### Prepare
```cd fission-rust-handler```  
```rm ../fission-rust-handler.zip```  
```zip -r ../fission-rust-handler.zip ./* ```

zip output should look like:
```shell
  adding: Cargo.lock (deflated 75%)
  adding: Cargo.toml (deflated 25%)
  adding: src/ (stored 0%)
  adding: src/lib.rs (deflated 24%)
```
### create environment
```fission env create --name rust-env --image glutamat/rust-env-1.57:dev --builder glutamat/rust-builder-1.57:dev --poolsize=1```  

Check env started:
```watch kubectl -n fission-builder get pods```

### create function
from the rust directory:  
```fission fn create --name fission-rust-handler --sourcearchive examples/fission-rust-handler.zip --env rust-env``` 

watch creating:
```watch fission pkg list```


