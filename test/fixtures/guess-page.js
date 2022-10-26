/**
 * @file 猜测 page 配置项
 * @author mengke01(kekee000@gmail.com)
 */
/* eslint-disable */
const guess = {
    wrapOptions(options) {
        return options;
    },
};

Page(guess.wrapOptions({
    data: {
        data1: 1
    },
    onLoad() {
        // TODO
    }
}));
