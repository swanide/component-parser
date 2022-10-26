/**
 * @file tabs
 * @author swan
 */
import {isIos, systemInfo} from '../../../common/utils/index';
/* global Component, swan */
/* eslint-disable babel/new-cap */
Component({
/* eslint-enable babel/new-cap */
    externalClasses: ['zw-tabs-nav-class', 'zw-tabs-nav-item-class'],
    properties: {
        className: {
            type: String,
            value: ''
        },
        type: {
            type: String,
            value: 'flex'
        },
        mode: {
            type: String,
            value: 'auto'
        },
        tabs: {
            type: Array,
            value: [],
            observer: function (newData, oldData) {
                if (newData !== oldData && newData.length && oldData.length) {
                    this.init(newData.length - 1);
                }
            }
        },
        activeTab: {
            type: Number,
            value: 0,
            observer: function (newData, oldData) {
                if (newData !== oldData && oldData !== 0) {
                    this.setData({
                        transitionFlag: false
                    }, () => {
                        this.init(newData, true);
                    });
                }
            }
        },
        onChange: {
            type: Function,
            value: () => {}
        },
        navBarWidth: {
            type: String,
            value: '100%'
        },
        navItemClassName: {
            type: String,
            value: ''
        },
        selectNavItemClassName: {
            type: String,
            value: ''
        },
        slideBarClassName: {
            type: String,
            value: ''
        },
        showWithAnimation: {
            type: Boolean,
            value: true
        }
    },
    data: {
        navItemWidth: 0,
        navbarInfo: {},
        sildeInfo: {},
        windowInfo: {},
        currentIndex: 0,
        scrollLeft: 0,
        slideLeft: 0,
        viewHeight: 0,
        scrollTop: 0,
        displaySwitch: false,
        allNavItemInfo: [],
        transitionFlag: false,
        isIos,
        initFinished: false
    },
    lifetimes: {
        attached() {
            const {activeTab} = this.data;
            // 初始化switch函数
            this.switchNav = this.switchNav();
            // 初始化数据
            this.init(activeTab, true);
        }
    },
    methods: {
        init(activeTab, isInit) {
            this.getDomInfo(['navbarInfo', 'sildeInfo', 'windowInfo', 'allNavItemInfo'])
            .then(res => {
                const [navbarInfo, sildeInfo, windowInfo, allNavItemInfo] = res;
                const navItemWidth = allNavItemInfo[activeTab].width || 0;
                const slideLeft = (navItemWidth - sildeInfo.width) / 2 + navItemWidth * activeTab;
                const viewHeight = windowInfo.windowHeight;
                const eventObj = {currentTarget: {dataset: {index: activeTab}}};
                this.setData({
                    navbarInfo,
                    navItemWidth,
                    sildeInfo,
                    windowInfo,
                    slideLeft,
                    allNavItemInfo,
                    viewHeight
                }, () => {
                    // 切换自定义tab
                    this.switchNav(eventObj, isInit);
                });
            });
        },
        switchNav() {
            let timeout = null;
            let isSwitching = false;
            return function (e, isInit) {
                const index = +e.currentTarget.dataset.index;
                const {currentIndex} = this.data;
                if (index === currentIndex && !isInit) {
                    return;
                }

                if (!isSwitching) {
                    // 节流锁
                    isSwitching = true;
                    clearTimeout(timeout);
                    timeout = setTimeout(() => {
                        isSwitching = false;
                    }, 300);

                    const {
                        navbarInfo,
                        sildeInfo,
                        windowInfo,
                        allNavItemInfo
                    } = this.data;
                    const navItem = allNavItemInfo[index];
                    const scrollLeft = navItem.left + navItem.width / 2 - navbarInfo.width / 2;
                    const myEventDetail = {index};
                    const computeLeft = this.computeLeft(allNavItemInfo, index);
                    this.setData({
                        scrollTop: -1
                    });
                    this.setData({
                        currentIndex: +index,
                        slideLeft: Math.abs(navItem.width - sildeInfo.width) / 2 + computeLeft,
                        scrollTop: 0,
                        scrollLeft: scrollLeft > 0 ? scrollLeft : 0,
                        displaySwitch: !+index
                    }, () => {
                        if (isInit) {
                            if (this.data.showWithAnimation) {
                                this.setData({
                                    transitionFlag: true,
                                    initFinished: true
                                });
                            }
                            else {
                                this.setData({
                                    initFinished: true
                                }, () => {
                                    setTimeout(() => {
                                        this.setData({
                                            transitionFlag: true
                                        });
                                    }, 200);
                                });
                            }
                        }
                        else {
                            this.triggerEvent('onChange', myEventDetail);
                        }
                    });
                }
            };
        },
        query(className, callback) {
            const query = swan.createSelectorQuery().in(this);
            query.select(className).boundingClientRect(res => {
                callback(res);
            });
            query.exec();
        },
        queryAll(className, callback) {
            const query = swan.createSelectorQuery().in(this);
            query.selectAll(className).boundingClientRect(res => {
                callback(res);
            });
            query.exec();
        },
        getDomInfo(keys) {
            return Promise.all(keys.map(key =>
                new Promise((resolve, reject) => {
                    this.getInfoMap()[key].method(data => {
                        resolve(data);
                    });
                })
            ));
        },
        computeLeft(allNavItemInfo, index) {
            return allNavItemInfo.reduce((total, current, idx) => {
                if (idx < index) {
                    return total + current.width;
                }
                else {
                    return total;
                }
            }, 0);
        },
        getInfoMap() {
            return {
                navbarInfo: {
                    method: callback => {
                        this.query('.tabs-nav', callback);
                    }
                },
                allNavItemInfo: {
                    method: callback => {
                        this.queryAll('.nav-item', callback);
                    }
                },
                navItemWidth: {
                    method: callback => {
                        this.query('.nav-item', callback);
                    }
                },
                sildeInfo: {
                    method: callback => {
                        this.query('.slide-border', callback);
                        this.triggerEvent('close', 1);
                    }
                },
                windowInfo: {
                    method: callback => {
                        callback(systemInfo && systemInfo || 0);
                    }
                }
            };
        }
    }
});
