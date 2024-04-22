
### alt-create

    根据模板创建前端新项目

### 功能

#### 注册模板
1. 支持从git获取模板
  ```sh
    alt-create -g 模板项目git地址 register 模板名称
  ```
2. 支持从本地获取模板
  ```sh
    alt-create -t 模板项目绝对地址 register 模板名称
  ```
    注册后模板会被缓存到temp目录

#### 删除已注册模板
  ```sh
    alt-create remove 模板名称
  ```
#### 查看已注册模板列表
  ```sh
    alt-create list
  ```
#### 更新模板
  ```sh
    alt-create update 可选：模板名称
  ```
  不输入模板名称时更新全部

#### 创建新项目
  ```sh
    alt-create create
  ```

