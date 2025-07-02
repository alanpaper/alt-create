

    根据模板创建前端新项目 rust实现

### alt-create

#### use

1. 拉取该项目文件
2. 执行`cargo run -r`
3. 创建alt-create文件夹
4. 将`target/release/`下的`alt-create`文件复制到alt-create
5. 添加alt-create可执行文件的环境变量

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

#### 创建新项目
  ```sh
    alt-create create
  ```

#### 解析当前文件夹下的markdown文件

  ```md
    title: dome, 
    category: dome, 
    tags: dome,
    outstanding: false,
    <!-- -----split----- -->
    // TODO content
  ```

  ```sh
    alt-create markdown
  ```

#### 添加小游戏

```
  alt-create play;
```


#### 局域网文件传输

1. 需要接受文件的设备 在需要接受文件的文件夹下执行

  ```sh
    alt-create transmit-server;
  ```

2. 另一台设备 执行

  ```sh
    alt-create transmit 文件路径 -ip 需要接受文件的ip地址;
  ```

#### 初始化deepseek翻译

  ```sh
    alt-create init deepseekAuthKey;
  ```

#### 翻译
  ```sh
    alt-create ask 测试;
  ```
