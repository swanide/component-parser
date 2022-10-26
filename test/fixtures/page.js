/**
 * @file 此文件为 Rust 模块测试文件，修改此文件，需要同时修改测试用例
 * @author mengke01(kekee000@gmail.com)
 */
Page({
    data: {
        // data1 属性
        data1: {
            data11: {
                /**
                 * data111
                 * @type {number} number
                 */
                data111: 1
            },
            data12: "zzz"
        },
        /**
         * data2
         */
        data2: null,
        get data3() {
            return 1;
        },
        "data4": void 0,
        5: 5
    },

    // method1 方法
    method1: () => {},
    method2: function () {},
    method3: async () => {},
    method4: async function () {},
    method5() {},
    async method6() {}
});