# poem_cloud


## 生成模型文件
```
cd app/src/
```

```
sea-orm-cli generate entity \
    -u mysql://root:root123123@127.0.0.1:3306/mysql \
    -o models

or 

sea-orm-cli generate entity -o models

sea-orm-cli migrate init
sea-orm-cli migrate -h
sea-orm-cli-migrate 
Migration related commands

USAGE:
    sea-orm-cli migrate [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
    -v, --verbose    Show debug messages
    -h, --help       Prints help information

OPTIONS:
    -d, --migration-dir <MIGRATION_DIR>    Migration script directory [default: ./migration]

SUBCOMMANDS:
    down        Rollback applied migrations
    fresh       Drop all tables from the database, then reapply all migrations
    generate    Generate a new, empty migration
    help        Prints this message or the help of the given subcommand(s)
    init        Initialize migration directory
    refresh     Rollback all applied migrations, then reapply all migrations
    reset       Rollback all applied migrations
    status      Check the status of all migrations
    up          Apply pending migrations
```

