/**
 * @file 入口定义
 * @author mengke01(kekee000@gmail.com)
 */

interface Pos {
    line: number;
    column: number;
}

interface Location {
    start: Pos;
    end: Pos;
}

interface DataMeta {
    name: string;
    loc: Location;
    comment?: string;
    children?: DataMeta[];
}

interface PropertyMeta {
    name: string;
    loc: Location;
    type: 'Boolean' | 'Number' | 'String' | 'Object';
    value?: string | number | boolean;
    comment?: string;
}

interface MethodMeta {
    name: string;
    loc: Location;
    comment?: string;
}

interface EventMeta {
    name: string;
    loc: Location;
    comment?: string;
}

interface ComponentMeta {
    /**
     * JS 文件类型
     */
    type: 'Component' | 'Page';

    /**
     * 数据定义
     */
    data: DataMeta[];

    /**
     * 方法定义
     */
    methods: MethodMeta[];

    /**
     * 自定义组件属性定义
     */
    properties: PropertyMeta[];

    /**
     * 绑定事件定义
     */
    events?: EventMeta[];
}

interface ClassNameMeta {
    name: string;
    loc: Location;
}

interface CssMeta {

    /**
     * class 数组
     */
    classes: ClassNameMeta[];

    /**
     * 导入的文件数组
     */
    imports: string[];
}

/**
 * 解析单个 js 文件
 * @param file 文件路径
 */
export function parseScript(file: string): ComponentMeta;

/**
 * 解析一组 js 文件
 * @param files 文件路径数组
 */
export function parseScriptFiles(files: string[]): Record<string, ComponentMeta>;

/**
 * 解析单个 css 文件
 * @param file 文件路径
 */
export function parseCss(file: string): CssMeta;

/**
 * 解析一组 css 文件，注意返回的 map 中包含所有被 import 的 css 文件
 * @param files 文件路径数组
 */
export function parseCssFiles(files: string[]): Record<string, CssMeta>;
