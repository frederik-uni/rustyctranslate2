# rustyctranslate2

## Requirements
- cmake
- git

### MacOS arm64
/

### MacOS x86_64
- oneapi
- onednn

### Linux arm64
- libopenblas-dev

### Linux x86_64
- oneapi
- oneapi-dnnl/onednn
- openmpi
- cuda
- nncl

### Windows x86_64
- oneapi
- onednn
- cuda
- cuDNN

## Info
A simple project that wraps around [CTranslate2](https://github.com/OpenNMT/CTranslate2)

https://github.com/OpenNMT/CTranslate2/tree/master/python/tools

How to use?
```rs
let model = CTranslator::new(PathBuf::from_str("...").unwrap(), false);
let tokens = ["▁H", "ell", "o", "▁world", "!"].into_iter().map(|v| v.to_string()).collect();
let v = model.unwrap().translate_batch(vec![tokens], None, BatchType::Example);
println!("{:?}", v);
```
