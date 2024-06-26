# getcontent - утилита для извлечения содержимого файлов

`getcontent` - это консольная утилита на Rust, предназначенная для Windows, которая рекурсивно извлекает содержимое файлов в указанной директории, предоставляя возможность фильтрации по именам файлов, использования шаблонов для распространенных кейсов,  и управления выводом.

### Возможности

- Рекурсивное сканирование директорий для поиска файлов.
- Фильтрация файлов по включаемым и исключаемым именам.
- Использование шаблонов для исключения файлов в распространенных проектах (например, Python, Rust).
- Копирование извлеченного содержимого в буфер обмена.
- Управление выводом в консоль (включая режим тишины).

### Установка

1. Убедитесь, что у вас установлен [Rust](https://www.rust-lang.org/tools/install) и Cargo.
2. Склонируйте репозиторий проекта:
   ```bash
   git clone https://github.com/f0rgenet/getcontent.git
   ```
3. Перейдите в директорию проекта:
   ```bash
   cd getcontent
   ```
4. Скомпилируйте проект:
   ```bash
   cargo build --release
   ```
5. Исполняемый файл будет находиться в директории `target/release/`.

### Использование

```
getcontent [Флаги]
```

#### Флаги

- `-e, --exclude <ФАЙЛЫ>`: Список имен файлов для исключения, разделенных пробелом.
- `-i, --include <ФАЙЛЫ>`: Список имен файлов для включения, разделенных пробелом.
- `-c, --copy`: Копировать извлеченное содержимое в буфер обмена.
- `-s, --silent`: Режим тишины - подавить вывод в консоль, кроме сообщений об ошибках и успешном копировании.
- `-t, --template <ШАБЛОН>`: Использовать шаблон для исключения файлов. Доступные шаблоны: `python`, `rust`.

#### Примеры

1. **Извлечение содержимого с исключениями и копированием в буфер обмена:**
   ```bash
   cd D:\\Projects\\Rust\\Sorting
   getcontent -exclude .idea target .gitignore Cargo.lock -copy
   ```
   **Вывод:**
   ```
   [./Cargo.toml]
   ```content
   [package]
   name = \"sortviz\"
   version = \"0.1.0\"
   edition = \"2021\"

   # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

   [dependencies]
   iced = { version = \"0.4\", features = [\"canvas\", \"tokio\", \"debug\", \"glow\"] }
   rand = \"0.8.5\"
   ```
   [./src/main.rs]
   ```content
   use iced::{Application, Settings};
   use crate::visualizer::SortingVisualizer;

   mod visualizer;
   mod algorithms;
   mod controls;
   mod painting;


   fn main() {
       let _ = SortingVisualizer::run(Settings {
           antialiasing: true,
           ..Settings::default()
       });

   }
   ```
   Содержимое [\"/Cargo.toml\", \"./src/main.rs\"] скопировано в буфер обмена!

2. **Извлечение определенного файла с копированием и тихим режимом:**
   ```bash
   cd D:\\Projects\\Rust\\Sorting
   getcontent -include main.rs -copy -silent
   ```
   **Вывод:**
   ```
   Содержимое [\"./src/main.rs\"] скопировано в буфер обмена!
   ```

3. **Использование шаблона для исключения файлов:**
   ```bash
   getcontent -t rust -c
   ```
   Этот код пропустит файлы и директории, типичные для проектов Rust (`target/`, `.idea`, `.gitignore`, `Cargo.lock`), и скопирует содержимое остальных файлов в буфер обмена.

### Приоритет флагов

- Флаг `-i` (`--include`) имеет приоритет над `-e` (`--exclude`) и `-t` (`--template`).
- Флаги `-e` и `-t` работают совместно, добавляя файлы и директории в список исключений.
- Если файл указан как в `-i`, так и в `-e` или в шаблоне, он будет обработан.
- Если ни один из флагов `-i`, `-e` или `-t` не указан, будут обработаны все файлы.

### Примечания

- Пути к файлам в выводе будут относительными к текущей директории.
- Для работы с буфером обмена требуется библиотека `clipboard`.

### Лицензия

Этот проект распространяется по лицензии MIT.