/**
 * @file 入口
 * @author mengke01(kekee000@gmail.com)
 */

const parser = (() => {
    if (process.platform === 'win32') {
        return require('./parser-x64-win32.node');
    }
    else if (process.platform === 'darwin' && process.arch === 'x64') {
        return require('./parser-x64-darwin.node');
    }
    else if (process.platform === 'darwin' && (process.arch === 'arm64')) {
        return require('./parser-aarch64-darwin.node');
    }

    return require('./parser-mock.js');
})();

exports.parseScript = filePath => {
    const result = parser.parseScript(filePath);
    return JSON.parse(result);
};

exports.parseScriptFiles = filePaths => {
    if (!Array.isArray(filePaths)) {
        throw new Error('file paths should be array!');
    }
    const result = parser.parseScriptFiles(filePaths);
    return JSON.parse(result);
};

exports.parseCss = filePath => {
    const result = parser.parseCss(filePath);
    return JSON.parse(result);
};

exports.parseCssFiles = filePaths => {
    if (!Array.isArray(filePaths)) {
        throw new Error('file paths should be array!');
    }
    const result = parser.parseCssFiles(filePaths);
    return JSON.parse(result);
};
