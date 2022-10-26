const assert = require('assert');

const testFiles = [
    [`${__dirname}/fixtures/component.js`, 'component'],
    [`${__dirname}/fixtures/page.js`, 'page'],
    [`${__dirname}/fixtures/complex-page.js`, 'page'],
    [`${__dirname}/fixtures/complex-component.js`, 'component'],
    [`${__dirname}/fixtures/error.js`, 'component'],
    [`${__dirname}/fixtures/error.js`, 'page'],
];


async function parseTestSwc() {
    const {parseScript} = require('../');
    const result = parseScript(`${__dirname}/fixtures/component.js`);
    console.log(JSON.stringify(result, null, 2));
}

async function parseTestAcorn() {
    const {parseScript} = require('./parser-acorn');
    const result = await parseScript(`${__dirname}/fixtures/component.js`, 'component');
    console.log(JSON.stringify(result, null, 2));
}


async function parseBySwc() {
    const {parseScriptFiles} = require('../');
    const result = parseScriptFiles(testFiles.map(([filePath]) => filePath));
    assert.strictEqual(Object.keys(result).length, 4, "解析成功个数错误！");
}

async function parseByAcorn() {
    const {parseScript} = require('./parser-acorn');
    const result = {};
    await Promise.all(testFiles.map(async ([filePath, type]) => {
        const res = await parseScript(filePath, type);
        if (res) {
            result[filePath] = res;
        }
    }));
    assert.strictEqual(Object.keys(result).length, 4, "解析成功个数错误！");
}

async function main() {
    const rounds = 10;
    console.time('parse:swc');
    for (let i = 0; i < rounds; i++) {
        await parseBySwc();
    }
    console.timeEnd('parse:swc');

    console.time('parse:acorn');
    for (let i = 0; i < rounds; i++) {
        await parseByAcorn();
    }
    console.timeEnd('parse:acorn');
}

main()
