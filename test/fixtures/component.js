/**
 * @file 此文件为 Rust 模块测试文件，修改此文件，需要同时修改测试用例
 * @author mengke01(kekee000@gmail.com)
 */
Component({
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
    properties: {

        /**
         * Boolean 属性
         */
        theme: {
            type: Boolean,
            value: false
        },

        /**
         * String 属性
         */
        "color-string": {
            type: String,
            value: '#3388FF'
        },

        /**
         * Number 属性
         */
        666: {
            type: Number,
            value: 666
        },

        /**
         * Object 属性
         */
        typeObject: {
            type: Object,
            value: {}
        },

        /**
         * Function 属性
         */
        typeFunction: {
            type: Function,
            value: () => {}
        },
        method1: () => {},
    },
    /* eslint-disable quotes */
    methods: {
        data: 1,

        // method1 方法
        method1: () => {
            // event1 事件
            this.triggerEvent('event1', 1);
        },
        method2: function () {
            this.triggerEvent(
                "event2", "event2");
            this.triggerEvent('event1', 1);
        },
        method3: async () => {},
        method4: async function () {},
        method5() {},
        async method6() {},
    }
});
