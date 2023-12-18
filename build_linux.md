# static linking tries
1. install autoconf, libtool
2. do cargo vcpkg install (install cargo-vcpkg first)
3. go to target/vcpkg/ports/libsystemd/portfile.cmake and comment out line 44-49 and line 52
4. rerun vcpkg install again
5. build with release mode (cargo build -r)