# This project is a Work in Progress

# Overview
This project aims to bring [Redis](https://redis.io/) functionality to DataFlex projects.

# Compilation
Install rust toolchain [Rustup](https://rustup.rs/)

Run build.bat

# Installation
Add library to project through Tools -> Maintain Libraries

Include the file with
```
Use DFRedisBinding.src
```

Copy the DLL `df_redis_binding.dll` to your programs directory or a global path.

# Usage
Create a connection object. I suggest doing this globally for a long lived object if you will be calling often.

```
Object oRedis is a cRedis
    Set psHost to "redis://localhost:6379"
End_Object
```

## Setting a value
```
Send SetValue to oRedis "Testing" "Works"
```

## Get a value
```
Get GetValue of oRedis "Testing" to _sResult
```