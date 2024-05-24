title: rullup打包项目配置
category: doc
<!-- -----split----- -->
### 配置一个基于pnpm + react + rullup + typescript的组件开发库项目

#### 首先基于pnpm新建一个空项目

```sh
  pnpm init
```

#### 安装所需模块

```sh
  # 组件开发相关
  pnpm add react typescript rxjs

  # 打包相关
  pnpm add rollup rollup-plugin-typescript2 @rollup/plugin-commonjs @rollup/plugin-node-resolve @rollup/plugin-terser --save-dev

  # 代码lint
  pnpm add eslint eslint-plugin-react eslint-plugin-react-hooks eslint-plugin-simple-import-sort --save-dev
  pnpm add husky lint-staged  --save-dev

  # 代码format
  pnpm add @biomejs/biome --save-dev

```

#### lint及format相关配置

- @biomejs/biome基于rust的开发的format工具(使用默认配置)

-- tsconfig.json
```json
{
  "compilerOptions": {
    "sourceMap": true,
    "module": "es2015",
    "target": "es2018",
    "moduleResolution": "node",
    "outDir": "./dist",
    "jsx": "react",
    "skipLibCheck": true,
    "declaration": true,
    "declarationMap": true,
    "noEmit": true,
    "esModuleInterop": true,
    "lib": ["dom", "dom.iterable", "esnext"],
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noImplicitReturns": true,
    "noFallthroughCasesInSwitch": true
  },
  "include": ["src"],
  "exclude": [
    "node_modules",
  ]
}
```


##### 配置eslintrc
```json
	{
	"parser": "@typescript-eslint/parser",
	"plugins": ["@typescript-eslint", "react-hooks", "simple-import-sort"],
	"extends": [
		"plugin:react/recommended",
		"plugin:@typescript-eslint/recommended"
	],
	"parserOptions": {
		"ecmaVersion": 2020,
		"sourceType": "module",
		"ecmaFeatures": {
			"jsx": true
		}
	},
	"rules": {
		"curly": "error",
		"no-extra-boolean-cast": "error",
		"cypress/unsafe-to-chain-command": "off",
		"@typescript-eslint/no-non-null-assertion": "off",
		"@typescript-eslint/no-empty-function": "off",
		"@typescript-eslint/ban-ts-comment": "warn",
		"@typescript-eslint/ban-types": "off",
		"@typescript-eslint/no-explicit-any": "off",
		"@typescript-eslint/explicit-module-boundary-types": "off",
		"@typescript-eslint/no-object-literal-type-assertion": "off",
		"@typescript-eslint/no-unused-vars": [
			"warn",
			{ "ignoreRestSiblings": true }
		],
		"cypress/no-unnecessary-waiting": "off",
		"react-hooks/rules-of-hooks": "error",
		"react-hooks/exhaustive-deps": "error",
		"react/display-name": "warn",
		"react/prop-types": "off",
		"no-console": ["error"],
		"simple-import-sort/imports": [
			"error",
			{
				"groups": [
					// Side effect imports.
					["^\\u0000"],
					// Packages. `react` related packages come first.
					["^react", "^@?\\w"],
					// Parent imports. Put `..` last.
					["^\\.\\.(?!/?$)", "^\\.\\./?$"],
					// Other relative imports. Put same-folder imports and `.` last.
					["^\\./(?=.*/)(?!/?$)", "^\\.(?!/?$)", "^\\./?$"]
				]
			}
		],
		"simple-import-sort/exports": "error"
	},
	"overrides": [
		{
			"files": ["*.test.ts", "*.test.tsx"],
			"rules": {
				// Allow testing runtime errors to suppress TS errors
				"@typescript-eslint/ban-ts-comment": "off"
			}
		}
	],
	"settings": {
		"react": {
			"pragma": "React",
			"version": "detect"
		}
	}
}

```


- husky及lint-staged使用配置 参考官网文档 pre-commit文件配置
```sh
#!/usr/bin/env sh
. "$(dirname -- "$0")/_/husky.sh"

pnpm lint-staged
```

- @biomejs/biome + lint-staged 相关配置, package.json文件添加如下代码

```json
	"scripts": {
    ...,
		"lint": "eslint '**/*.{js,ts,tsx}'",
		"lint:fix": "pnpm lint --fix",
		"prepare": "husky install"
	},


	"lint-staged": {
		"*.{js,ts,d.ts,jsx,tsx,json}": [
			"biome check --files-ignore-unknown=true",
			"biome check --apply --no-errors-on-unmatched",
			"biome check --apply --organize-imports-enabled=false --no-errors-on-unmatched",
			"biome check --apply-unsafe --no-errors-on-unmatched",
			"biome format --write --no-errors-on-unmatched",
			"biome lint --apply --no-errors-on-unmatched",
			"pnpm lint:fix"
		],
		"*.{md,yml}": ["biome format --write --no-errors-on-unmatched"]
	},
	"husky": {
		"hooks": {
			"pre-commit": "lint-staged"
		}
	}
```

#### rollup.config.js配置
```js
import commonjs from "@rollup/plugin-commonjs";
import resolve from "@rollup/plugin-node-resolve";
import terser from "@rollup/plugin-terser";
import typescript from "rollup-plugin-typescript2";

import pkg from "../package.json" assert { type: "json" };

export function createRollupConfig(options, callback) {
	const name = options.name;

	// A file with the extension ".mjs" will always be treated as ESM, even when pkg.type is "commonjs" (the default)
	// https://nodejs.org/docs/latest/api/packages.html#packages_determining_module_system
	const extName = options.format === "esm" ? "mjs" : "js";

	const outputName = `dist/${[name, options.format, extName].join(".")}`;

	const config = {
		input: options.input,
		output: {
			file: outputName,
			format: options.format,
			name: "AlanUi",
			sourcemap: true,
			// globals: { react: "React", rxjs: "rxjs" },
			exports: "named",
		},
		plugins: [
			resolve(),
			typescript({
				tsconfig: options.tsconfig,
				clean: true,
				exclude: ["**/__tests__", "**/*.test.ts"],
			}),
			commonjs({
				include: /\/node_modules\//,
			}),
			options.format !== "esm" &&
				terser({
					output: { comments: false },
					compress: {
						drop_console: true,
					},
				}),
		].filter(Boolean),
	};

	return callback ? callback(config) : config;
}

const name = "index";
const options = [
	{
		name,
		format: "cjs",
		input: pkg.source,
	},
	{ name, format: "esm", input: pkg.source },
	{
		name,
		format: "umd",
		input: pkg.source,
	},
];

export default options.map((option) => createRollupConfig(option));

```


### 测试组件(button)

- 默认出入与导出的问题 这里采用导出由于默认导出不支持 `export * from "./button"`语法

```tsx
import React, { useEffect, useState } from "react";
import { range, takeLast } from "rxjs";

interface Iprops {
	type: "waring" | "danger";
}
export const AyButton = React.forwardRef(function Button(
	props: Iprops,
	ref: React.ForwardedRef<HTMLDivElement>,
) {
	const { type } = props;
	const [state, setState] = useState(0);
	useEffect(() => {
		const many = range(1, 100);
		const lastThree = many.pipe(takeLast(3));
		lastThree.subscribe((x) => {
			setState(x);
		});
	}, []);

	return (
		<div ref={ref} className="button">
			测试按钮 {type} {state}
		</div>
	);
});
```



### 代码地址
[地址]()