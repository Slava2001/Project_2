# Project_2
2d sandbox

## Build
### Windows
```
cmake -G "MinGW Makefiles" -S .\ -B.\build
cmake --build .\build 
cmake --install .\build --prefix .\Project2 
```
### Linux
```
cmake -S ./ -B ./build
cmake --build ./build
cmake --install ./build --prefix ./Project2 
export LD_LIBRARY_PATH=./Project2/
./Project2/Project_2
```