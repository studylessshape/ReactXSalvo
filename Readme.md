# ReactXSalvo

## Use Framework

- [frontend](./frontend/)
  - [React](https://react.dev/)
  - [Ant Desgin](https://ant-design.antgroup.com/)
- [backend](./server/)
  - [Salvo](https://salvo.rs/)

## Require

- [frontend](./frontend/)
  - [Nodejs](https://nodejs.org/) >= 20
    - If use bun: [bun](https://bun.sh/) >= 1.2
- [backend](./server/)
  - [Rust](https://www.rust-lang.org/) >= 1.82

## Build Frontend

Run build script and all executable files is in `build` folder.

### Use Nodejs

Run [node-build.sh](./node-build.sh) or [node-build.bat](./node-build.bat)

### Use bun

Run [bun-build.sh](./bun-build.sh) or [bun-build.bat](./bun-build.bat)

## Build Server

### Prepair

#### Windows

- [Visual Studio](https://visualstudio.microsoft.com/)
- [CMake](https://cmake.org/)
- [Ninja](https://ninja-build.org/)
- [vcpkg](https://vcpkg.io/)
  - [openssl](https://www.openssl.org/): installed by vcpkg

1. Install [Visual Studio]((https://visualstudio.microsoft.com/)): Use the Visual Studio Installer to install the `Desktop development with C++` workload. Ensure it includes the latest MSVC (Microsoft Visual C++) build tools and Windows SDK .

2. Install [CMake](https://cmake.org/): Download and install CMake, then add its executable to your system's PATH environment variable during installation .

3. Install [Ninja](https://ninja-build.org/):

Download the latest release from [Ninja GitHub Releases](https://github.com/ninja-build/ninja/releases). Extract the package and add the directory containing the ninja.exe executable to your PATH.

4. Install [vcpkg](https://vcpkg.io/):

```powershell
git clone https://github.com/microsoft/vcpkg.git
cd vcpkg
.\bootstrap-vcpkg.bat
```

Set the environment variable VCPKG_ROOT to point to your vcpkg directory (e.g., C:\dev\vcpkg) .

5. Install OpenSSL:

```powershell
vcpkg install openssl
```

6. Configure Environment Variables:

Add a system environment variable named OPENSSL_DIR with the value:

```
%VCPKG_ROOT%\installed\x64-windows
```

Ensure `VCPKG_ROOT` is properly set if you followed the standard vcpkg installation procedure.

#### Linux

- Install [openssl](https://www.openssl.org/) by package manager. See [openssl-rs#automatic](https://docs.rs/openssl/latest/openssl/#automatic)