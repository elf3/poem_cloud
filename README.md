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
```