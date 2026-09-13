# Python Environment Adjuster

A small interactive Rust utility for checking Python installations and creating Python virtual environments. It is designed to help when the Windows Python launcher (`py`) does not recognise a Python version that is available through the `python` command.

This project is intended to be released under the MIT License. Add the `LICENSE` file with your own MIT licence text when you are ready.

## Features

- Checks whether Python is available.
- Checks whether the Python `venv` module can be used.
- Asks for a virtual-environment name and Python version.
- On Windows, tries `py -<version> -m venv <name>` first.
- If `py` cannot find the requested version, checks `python --version` and falls back to `python -m venv <name>` when the version matches.
- If the user asks to install a missing version on Windows, tries `py install <version>` first and then `winget`.
- Reports command failures instead of claiming that an operation succeeded.

## Requirements

- Rust and Cargo.
- Python installed and available through `python` or `py`.
- Windows: `winget` is optional and is only needed when the program must install a missing Python version.
- Linux: the current implementation expects `python<version>` and uses `pyenv` when it needs to install a version.

## Build and run

```powershell
cargo check
cargo run
```

Answer `y` when the program asks whether to start. To create an environment, provide a name such as `.venv` and a version such as `3.11`.

## Windows version-detection behaviour

`py` and `python` may refer to different Python installations. For example:

```text
py --version
Python 3.13.13

python --version
Python 3.11.9
```

In this situation, `py -3.11 -m venv .venv` can fail even though `python -m venv .venv` works. The program handles this by trying the launcher first and then checking the version returned by `python`.

The program does not silently change the requested version. The fallback is used only when the `python` command reports the exact requested major and minor version.

## Installing a missing Windows version

When the user chooses to install a version, the program attempts:

```powershell
py install 3.12
```

Some computers have the legacy `py.exe`, which does not support this command. In that case the program tries:

```powershell
winget install --id Python.Python.3.12 --exact --accept-source-agreements --accept-package-agreements
```

This requires an internet connection and a working `winget` installation. Windows may ask for confirmation or administrator permission. If both attempts fail, the virtual environment is not created and the error is shown to the user.

## Important limitations and warnings

- The program does not guarantee that Python is installed; it reports the result of the commands available on the machine.
- A Python version installed through `python` may not be registered with the Windows launcher. The fallback handles the common case, but PATH and file-association configuration can still affect the result.
- The requested version must match the detected major and minor version. For example, `3.11` matches `Python 3.11.9`.
- `venv` must be present in the selected Python installation.
- The program currently does not install packages inside the new environment.
- The program currently does not activate the environment automatically. On Windows, activation is normally run with `<environment>\\Scripts\\Activate.ps1`; on Linux, use `source <environment>/bin/activate`.
- User input is passed to external commands. Use simple environment names and version values such as `.venv` and `3.11`.
- The current interaction messages are in English, although this README is available in several languages.
- Run `cargo check` after changes. Warnings should be reviewed rather than ignored; a clean check is preferred.

## Troubleshooting

### `No suitable Python runtime found`

Run:

```powershell
py --list
python --version
```

If the requested version appears only in `python --version`, the program should use its fallback. Otherwise, install the requested version and run the program again.

### `No module named install`

This means an invalid command such as `py -m install <version>` was used. Python installation is not a normal Python module operation. The program uses `py install <version>` and then `winget` on Windows.

### `winget is not available`

Install or update App Installer from Microsoft, or install Python manually. Then confirm that the requested version works with:

```powershell
python --version
py --list
```

### The environment directory already exists

Choose another environment name or remove the old directory only after confirming that it contains no work you need.

## Project structure

```text
src/
  main.rs                 Interactive application flow
  utilidades/
    mod.rs                Python detection, installation and venv creation
```

## Licence

This project is intended to use the MIT License. The project author is responsible for adding and maintaining the final `LICENSE` file and copyright notice.

---

# Documentacion en Espanol

## Descripcion

Python Environment Adjuster es una utilidad interactiva escrita en Rust para comprobar instalaciones de Python y crear entornos virtuales. Esta pensada especialmente para el caso en Windows en el que `py` no reconoce una version que si esta disponible mediante el comando `python`.

El proyecto esta pensado para publicarse bajo la licencia MIT. El autor puede agregar el archivo `LICENSE` con su propio texto de licencia MIT.

## Que hace

- Comprueba si Python esta disponible.
- Comprueba si se puede usar el modulo `venv`.
- Solicita el nombre del entorno y la version de Python.
- En Windows intenta primero `py -<version> -m venv <nombre>`.
- Si `py` no encuentra la version, comprueba `python --version` y usa `python -m venv <nombre>` cuando la version coincide.
- Si el usuario solicita instalar una version, intenta `py install <version>` y despues `winget`.
- Muestra los errores reales y no informa de exito cuando una operacion falla.

## Requisitos y uso

Se necesita Rust/Cargo y una instalacion de Python accesible mediante `python` o `py`. En Windows, `winget` es opcional y solo se necesita para intentar instalar una version ausente.

```powershell
cargo check
cargo run
```

Responde `y` para iniciar el programa. Para crear un entorno, escribe un nombre como `.venv` y una version como `3.11`.

## Diferencia entre `py` y `python`

Es posible que ambos comandos apunten a instalaciones distintas. Por ejemplo, `py` puede detectar Python 3.13 y `python` puede ejecutar Python 3.11. En ese caso, `py -3.11 -m venv .venv` falla aunque `python -m venv .venv` funciona. El programa intenta resolver este caso con un fallback y comprueba la version mayor y menor antes de usarlo.

## Instalacion automatica en Windows

Si `py install <version>` no esta disponible porque se usa el lanzador antiguo, se intenta `winget`. Este proceso puede necesitar internet, confirmacion del usuario o permisos de administrador. Si no se puede instalar la version, el entorno no se crea.

## Advertencias

- La version solicitada debe coincidir con la version mayor y menor detectada; `3.11` coincide con `Python 3.11.9`.
- La instalacion seleccionada debe incluir `venv`.
- El programa no instala paquetes ni activa el entorno automaticamente.
- Usa nombres de entorno y versiones sencillos, como `.venv` y `3.11`.
- Los mensajes de la aplicacion estan actualmente en ingles.
- Ejecuta `cargo check` despues de modificar el codigo y revisa cualquier warning.

## Licencia

El proyecto esta destinado a utilizar la licencia MIT. El autor debe agregar y mantener el archivo final `LICENSE` y el aviso de copyright.

---

# 中文文档

## 项目简介

Python Environment Adjuster 是一个使用 Rust 编写的交互式工具，用于检查 Python 安装并创建 Python 虚拟环境。它特别适合处理 Windows 中 `py` 找不到某个 Python 版本、但 `python` 命令可以使用该版本的情况。

本项目计划采用 MIT 许可证发布。作者可以自行添加包含 MIT 许可证文本的 `LICENSE` 文件。

## 功能

- 检查 Python 是否可用。
- 检查 `venv` 模块是否可以使用。
- 询问虚拟环境名称和 Python 版本。
- 在 Windows 上优先尝试 `py -<版本> -m venv <名称>`。
- 如果 `py` 找不到指定版本，则检查 `python --version`；版本匹配时使用 `python -m venv <名称>`。
- 用户请求安装缺少的版本时，先尝试 `py install <版本>`，然后尝试 `winget`。
- 失败时显示实际错误，不会错误地显示成功信息。

## 要求与运行

需要安装 Rust/Cargo，并且 Python 可以通过 `python` 或 `py` 访问。在 Windows 上，只有需要自动安装缺少的 Python 版本时才需要 `winget`。

```powershell
cargo check
cargo run
```

程序询问是否开始时输入 `y`。创建环境时，可以输入 `.venv` 作为名称，输入 `3.11` 作为版本。

## `py` 与 `python` 的区别

这两个命令可能指向不同的 Python 安装。例如，`py` 可能检测到 Python 3.13，而 `python` 可能运行 Python 3.11。因此 `py -3.11 -m venv .venv` 可能失败，但 `python -m venv .venv` 可以成功。程序会检查版本的主版本号和次版本号后再使用备用方式。

## 注意事项

- 请求的版本必须与检测到的主版本号和次版本号一致；`3.11` 可以匹配 `Python 3.11.9`。
- 被选中的 Python 安装必须包含 `venv`。
- 程序不会自动安装虚拟环境中的第三方包，也不会自动激活环境。
- 建议使用简单的环境名称和版本，例如 `.venv` 和 `3.11`。
- 当前应用程序界面消息为英文。
- 修改代码后请运行 `cargo check` 并检查 warning。

## 许可证

本项目计划采用 MIT 许可证。作者负责添加和维护最终的 `LICENSE` 文件以及版权声明。

---

# Документация на русском языке

## О проекте

Python Environment Adjuster — это интерактивная утилита на Rust для проверки установки Python и создания виртуальных окружений. Она особенно полезна в Windows, когда `py` не находит нужную версию Python, но команда `python` эту версию запускает.

Проект планируется распространять по лицензии MIT. Автор может самостоятельно добавить файл `LICENSE` с текстом лицензии MIT.

## Возможности

- Проверка доступности Python.
- Проверка возможности использовать модуль `venv`.
- Запрос имени виртуального окружения и версии Python.
- В Windows сначала выполняется `py -<версия> -m venv <имя>`.
- Если `py` не находит версию, проверяется `python --version`; при совпадении версии используется `python -m venv <имя>`.
- При запросе установки отсутствующей версии сначала используется `py install <версия>`, затем `winget`.
- При ошибке программа показывает настоящее сообщение и не сообщает об успешном выполнении.

## Требования и запуск

Нужны Rust/Cargo и Python, доступный через `python` или `py`. В Windows `winget` нужен только для попытки автоматической установки отсутствующей версии Python.

```powershell
cargo check
cargo run
```

На вопрос о запуске ответьте `y`. Для создания окружения можно указать `.venv` и версию `3.11`.

## Разница между `py` и `python`

Эти команды могут указывать на разные установки Python. Например, `py` может видеть Python 3.13, а `python` запускать Python 3.11. Поэтому `py -3.11 -m venv .venv` может завершиться ошибкой, хотя `python -m venv .venv` работает. Программа проверяет основную и дополнительную версии перед использованием резервного варианта.

## Предупреждения

- Запрошенная версия должна совпадать с основной и дополнительной версиями найденного Python; `3.11` соответствует `Python 3.11.9`.
- В выбранной установке Python должен быть доступен модуль `venv`.
- Программа не устанавливает сторонние пакеты и не активирует окружение автоматически.
- Используйте простые имена окружений и версии, например `.venv` и `3.11`.
- Сообщения приложения пока написаны на английском языке.
- После изменений запускайте `cargo check` и проверяйте warning.

## Лицензия

Проект планируется выпускать по лицензии MIT. Автор отвечает за добавление и поддержку окончательного файла `LICENSE` и уведомления об авторских правах.

---

# 日本語ドキュメント

## 概要

Python Environment Adjuster は、Python のインストール状態を確認し、Python の仮想環境を作成する Rust 製の対話型ツールです。Windows で `py` が特定の Python バージョンを見つけられない一方、`python` コマンドではそのバージョンを実行できる場合に役立ちます。

このプロジェクトは MIT ライセンスで公開する予定です。作者が MIT ライセンスの本文を含む `LICENSE` ファイルを追加できます。

## 機能

- Python が利用可能か確認します。
- `venv` モジュールを使用できるか確認します。
- 仮想環境の名前と Python のバージョンを尋ねます。
- Windows では最初に `py -<バージョン> -m venv <名前>` を試します。
- `py` が見つけられない場合、`python --version` を確認し、バージョンが一致すれば `python -m venv <名前>` を使用します。
- 不足しているバージョンのインストールを選択した場合、`py install <バージョン>` の後に `winget` を試します。
- 失敗時には実際のエラーを表示し、成功したとは表示しません。

## 必要条件と実行方法

Rust/Cargo と、`python` または `py` から利用できる Python が必要です。Windows で不足している Python を自動インストールする場合だけ `winget` が必要です。

```powershell
cargo check
cargo run
```

開始確認には `y` と入力します。仮想環境名には `.venv`、バージョンには `3.11` などを入力できます。

## `py` と `python` の違い

両方のコマンドが別々の Python インストールを指すことがあります。たとえば `py` は Python 3.13 を検出し、`python` は Python 3.11 を実行する場合があります。この場合、`py -3.11 -m venv .venv` は失敗しても、`python -m venv .venv` は成功する可能性があります。プログラムはメジャー番号とマイナー番号を確認してから代替方法を使います。

## 注意事項

- 指定したバージョンは検出されたメジャー番号とマイナー番号に一致する必要があります。`3.11` は `Python 3.11.9` と一致します。
- 選択した Python に `venv` が含まれている必要があります。
- このプログラムは仮想環境内のパッケージをインストールせず、環境も自動で有効化しません。
- `.venv` や `3.11` のような単純な名前とバージョンを使用してください。
- 現在のアプリケーションメッセージは英語です。
- コードを変更した後は `cargo check` を実行し、warning を確認してください。

## ライセンス

このプロジェクトは MIT ライセンスで公開する予定です。最終的な `LICENSE` ファイルと著作権表示は作者が追加・管理します。
