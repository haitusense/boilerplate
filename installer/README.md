# ht-install

Install .NET Core Tool / Rust DLL from github repository

## Description 

```ps1
ps> cd [install folder]
ps> git clone [url]/[repository].git
ps> dotnet publish ./[repository]/[target] -c Release -o ./
ps> rm -Force -R ./[repository]
```

↓

```ps1
ps> ht-install cs [url]/[repository].git -o "./"
```

### Dependencies / Required

- dotnet
- rustup (cargo)
- powershell
- git

**install on Windows**

```ps1
# install Powershell
ps> winget install --id Microsoft.Powershell --source winget
ps> $PSVersionTable

# install dotnet
ps> winget search Microsoft.DotNet
ps> winget install Microsoft.DotNet.SDK.8
ps> winget install Microsoft.DotNet.DesktopRuntime.8
ps> winget install Microsoft.DotNet.AspNetCore.8
ps> dotnet --list-sdks 
ps> dotnet --list-runtimes

# install rustup
ps> winget install Rustlang.Rustup

# install git
ps> winget install --id Git.Git -e --source winget
```

## Usage

**installation**

```ps1
# install from Github
ps> cargo install --git https://github.com/haitusense/boilerplate ht-install

# install from local
ps> cargo install --path ./installer
```

**examples**

```ps1
# help
ps> ht-install --help

# run
ps> ht-install cs https://github.com/haitusense/boilerplate.git -s "./cs" -o "./"
```

## reference

```ps1
# use dotnet-git-tool
ps> dotnet tool install dotnet-git-tool -g

# use csx code on GitHub
ps> dotnet script https://raw.githubusercontent.com/haitusense/boilerplate/main/cs
```