const testFiles = [
    [`${__dirname}/fixtures/component.js`, 'component'],
    [`${__dirname}/fixtures/page.js`, 'page'],
    [`${__dirname}/fixtures/complex-page.js`, 'page'],
    [`${__dirname}/fixtures/complex-component.js`, 'component'],
    [`${__dirname}/fixtures/error.js`, 'component'],
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


async function parseFilesBySwc() {
    const {parseScriptFiles} = require('../');
    console.time('parse:swc');
    const result = parseScriptFiles(testFiles.map(([filePath]) => filePath));
    console.timeEnd('parse:swc');
    console.log(Object.keys(result));
}

async function parseFilesByAcorn() {
    const {parseScript} = require('./parser-acorn');
    console.time('parse:acorn');
    const result = {};
    await Promise.all(testFiles.map(async ([filePath, type]) => {
        result[filePath] = await parseScript(filePath, type);
    }));
    console.timeEnd('parse:acorn');
    console.log(Object.keys(result));
}


async function parseCssFiles() {
    const {parseCss, parseCssFiles} = require('../');
    parseCss(`${__dirname}/fixtures/component.css`);
    console.time('parse:css');
    const result = parseCssFiles(
        [
            `${__dirname}/fixtures/component.css`,
            `${__dirname}/fixtures/page.css`
        ]
    );
    console.timeEnd('parse:css');
    console.log(Object.keys(result));
}

async function main() {
    await parseFilesBySwc();
    await parseFilesByAcorn();

    await parseCssFiles();
}

// main();
parseTestSwc();
