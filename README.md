# Introduction To Optimization / Programming Task 2

**by SDR**

![Python](https://img.shields.io/badge/python-3670A0?style=for-the-badge&logo=python&logoColor=ffdd54)
![Poetry](https://img.shields.io/badge/Poetry-%233B82F6.svg?style=for-the-badge&logo=poetry&logoColor=0B3D8D)

```shell
python3.13 -m venv .venv/
```

Activate the virtual environment.

```bash
source .venv/bin/activate
```

```shell
pip install poetry
poetry install
poetry run launch
```

---

[![Bun][Bun.js]][Bun-url] [![TypesScript][TypeScript-logo]][TypeScript-url]

> **Pay attention!**
>
> We solved this assignment using TypeScript with the Bun runtime. The code will
> not work with Node.js because we&CloseCurlyQuote;ve used the built-in testing
> library, unavailable on Node.

## Installation

You can install Bun using their
[official guide](https://bun.sh/docs/installation).

For MacOS and Linux:

```shell
curl -fsSL https://bun.sh/install | bash
```

For Windows:

```shell
powershell -c "irm bun.sh/install.ps1|iex"
```

## Run Locally

After cloning the repository, install dependencies.

```bash
bun install
```

To run the project, use the command below.

```bash
bun test
```

You may find the report under [report/report.pdf](/report/report.pdf).

## License

The project is protected under the [Student Open License](/LICENSE) to prevent
cheating from other students, and is also distributed under the
[BSD 3-Clause License](/LICENSE-BSD) for other users of the software.

[Bun.js]: https://img.shields.io/badge/Bun-%23000000.svg?style=for-the-badge&logo=bun&logoColor=white
[Bun-url]: https://bun.sh/
[TypeScript-logo]: https://img.shields.io/badge/typescript-000000.svg?style=for-the-badge&logo=typescript&logoColor=white
[TypeScript-url]: https://www.typescriptlang.org/
