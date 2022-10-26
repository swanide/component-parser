/**
 * @file script 辅助函数
 * @author mengke01(kekee000@gmail.com)
 */
const acorn = require('acorn');
const {promises: fs} = require('fs');

const findConfigPropertyMeta = (configNode, property) => {
    const travelObject = (parentNode, srcNode) => {
        const node = {
            // Identifier or Literal
            name: srcNode.key.type === 'Identifier' ? srcNode.key.name : srcNode.key.value,
            loc: srcNode.key.loc,
            children: null,
        };
        parentNode.children.push(node);
        if (srcNode.value.type === 'ObjectExpression') {
            node.children = [];
            for (const srcChild of srcNode.value.properties
                .filter(i => i.key.type === 'Identifier' || i.key.type === 'Literal')) {
                travelObject(node, srcChild);
            }
        }
    };

    const dataNode = configNode.properties.find(node =>
        node.key.name === property
        && node.value.type === 'ObjectExpression');
    if (dataNode) {
        const dataRoot = {
            name: dataNode.key.name,
            loc: dataNode.key.loc,
            children: [],
        };
        dataNode.value.properties.map(node => travelObject(dataRoot, node));
        return dataRoot.children;
    }
};

/**
 * 解析业务逻辑代码，返回 data 变量和 函数
 * @param src source code
 */
function parseScriptMeta(src, type) {
    const ast = acorn.parse(src, {
        ecmaVersion: 2020,
        sourceType: 'module',
        locations: true,
        ranges: false,
    });

    const mixInName = type === 'page' ? 'Page' : 'Component';
    const configNode = ast.body?.find(node =>
        node.type === 'ExpressionStatement'
        && node.expression.type === 'CallExpression'
        && node.expression.callee.name === mixInName
        && node.expression.arguments?.[0].type === 'ObjectExpression'
    )?.expression.arguments[0];

    if (!configNode) {
        return null;
    }

    if (type === 'page') {
        const pageMeta = {
            type,
            data: [],
            methods: [],
        };
        pageMeta.data = findConfigPropertyMeta(configNode, 'data');

        // methods
        {
            const functionNodes = configNode.properties.filter(node =>
                node.key.type === 'Identifier'
                && node.value.type === 'FunctionExpression');
            if (functionNodes) {
                pageMeta.methods = functionNodes.map(fnNode => ({
                    name: fnNode.key.name,
                    loc: fnNode.key.loc,
                }));
            }
        }
        return pageMeta;
    }
    else if (type === 'component') {
        const componentMeta = {
            type,
            data: [],
            properties: [],
            methods: [],
        };
        componentMeta.data = findConfigPropertyMeta(configNode, 'data');
        // properties
        {
            const propertyNodes = configNode.properties.find(node =>
                node.key.name === 'properties'
                && node.value.type === 'ObjectExpression')
                ?.value.properties
                .filter(i => i.key.type === 'Identifier' || i.key.type === 'Literal');
            if (propertyNodes) {
                componentMeta.properties = propertyNodes.map(node => ({
                    name: node.key.type === 'Identifier' ? node.key.name : node.key.value,
                    loc: node.key.loc,
                }));
            }
        }
        // methods
        {
            const functionNodes = configNode.properties.find(node =>
                node.key.name === 'methods'
                && node.value.type === 'ObjectExpression')
                ?.value.properties
                .filter(node => node.key.type === 'Identifier' && node.value.type === 'FunctionExpression');
            if (functionNodes) {
                componentMeta.methods = functionNodes.map(node => ({
                    name: node.key.name,
                    loc: node.key.loc,
                }));
            }
        }
        return componentMeta;
    }

    return null;
}

/**
 * 获取 swan 文件 script meta 信息
 * @param fileUri swan 文件地址
 * @param type 文件类型
 */
exports.parseScript = async function parseScript(fileUri, type) {
    const scriptPath = fileUri.replace(/\.\w+$/, '.js');

    // read from file
    try {
        let stats = await fs.stat(scriptPath);
        if (!stats.isFile()) {
            return null;
        }
    }
    catch (e) {
        console.warn('no swan script file', fileUri);
    }

    try {
        const scriptText = await fs.readFile(scriptPath, 'utf-8');
        return parseScriptMeta(scriptText, type);
    }
    catch (e) {
        console.warn('get script meta error', fileUri, e.message);
    }

    return null;
}
