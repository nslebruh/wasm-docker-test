# Simple HTTP web server built in Rust compiled to WebAssembly and hosted using Docker

## Prerequisites
- Docker desktop is installed
- ContainerD images for Docker desktop is enabled

### To enable ContainerD images for Docker desktop:
1. Open Docker desktop
2. Navigate to docker settings
3. Navigate to "Features in development"
4. Select "Use containerd for pulling and storing images"
5. Restart Docker desktop

## To run:
Make sure that if you open the dockerfile, you check that it has been saved using LF (Line Feed) and not CRLF (Carriage Return Line Feed). For VSCode, this can be seen and changed through the status bar

```prompt
git clone https://github.com/nslebruh/wasm-docker-test.git

cd ./wasm-docker-test

docker compose up
```



