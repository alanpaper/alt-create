title: 子目录中husky安装使用
category: doc
<!-- -----split----- -->
### 安装

```sh
$ npm install husky lint-staged prettier pretty-quick --save-dev


// 自动化生成.husky文件夹
$ npm pkg set scripts.prepare="cd ../../ && husky install /package/core/.husky"

// package.json添加
"scripts": {
  "pretty-quick": "pretty-quick --staged",
}
"husky": {
  "hooks": {
    "pre-commit": "npm run pretty-quick"
  }
},

$ npm run prepare;

$ npx husky add .husky/pre-commit "npm run pretty-quick";

// 打开当前目录下的.husky的pre-commit文件
// 添加cd命令 操作目录从.git的文件根目录到当前目录的地址
#!/usr/bin/env sh
. "$(dirname -- "$0")/_/husky.sh"


cd package/core/
npm run pretty-quick



// 完成后即可测试
$ git add .husky/pre-commit

```
