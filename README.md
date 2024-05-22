

    根据模板创建前端新项目 rust实现

### alt-create

#### 使用方式

1. 拉取该项目文件
2. 执行`cargo run -r`
3. 创建alt-create文件夹
4. 将`target/release/`下的`alt-create`文件复制到alt-create
5. 添加alt-create可执行文件的环境变量

    仅在mac环境下做过测试

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
