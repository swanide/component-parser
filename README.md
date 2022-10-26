# @swanide/component-parser

使用 Rust + swc 解析小程序 Page，Component 组件中的变量信息，给代码提示模块使用，多文件解析下，性能大概是 acorn 的 10 倍。

使用 Rust 解析 css 类名，支持 `@import` 多文件并行解析。


使用方法：

```javascript
import {parseFiles, parseFile, parseCss, parseCssFiles} from '@swanide/component-parser';

// 解析 script meta
const filePaths = [
    'test/fixtures/page.js',
    'test/fixtures/component.js'
];
// 解析多个文件
const result = parseFiles(filePaths);
console.log(result);
// 解析单个文件
const result = parseFile('test/fixtures/page.js');
console.log(result);

// 解析 css meta
const cssFiles = [
    'test/fixtures/page.css',
    'test/fixtures/component.css'
];
// 解析多个文件，注意返回的 map 中包含所有被 import 的 css 文件
const result = parseCssFiles(cssFiles);
console.log(result);
// 解析单个文件
const result = parseCss('test/fixtures/page.css');
console.log(result);
```
