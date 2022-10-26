/**
 * @file index.js
 * @author swan
 */
import {checkNetwork, checkFullScreen} from '../common/utils';
import {
    checkIdcard,
    checkPhone,
    checkEmpty
} from '../common/form';
import {mockData} from './utils/mock';
/* global Page, swan */

Page({
    data: {
        // 当前进度，1 表示第一步
        active: 1,
        // 判断是否为全面屏
        isFullScreen: checkFullScreen,
        // 加载loading，加载之前设置为 true，加载完成后设置为 false
        isPageLoading: false,
        // 错误页面状态
        pageResult: '',
        // 第四步信息集合
        informateFour: {},
        // 每个进度所对应的标题文案
        step: [
            {
                text: '标题一'
            },
            {
                text: '标题二'
            },
            {
                text: '标题三'
            },
            {
                text: '核对信息'
            }
        ],
        // 表单对应 value 值
        formData: {
            // 身份证号
            idcard: '',
            // 手机号
            phone: '',
            // 日期
            date: '',
            // 地址
            place: '',
            // 输入框一
            iptone: '',
            // 输入框二
            ipttwo: '',
            // 选择框一
            sltsingle: -1,
            // 联动选择
            sltdouble: [],
            // 单选
            radios: 0,
            // 多选
            checkbox: [0],
            // 不包含以上情况
            nochecked: false,
            // 长文本
            textBox: '',
            // 照片
            uploadphotos: []
        },
        // 输入框与选择框的错误状态
        errorInfo: {
            idcardError: '',
            phoneError: '',
            dateError: '',
            placeError: '',
            iptoneError: '',
            ipttwoError: '',
            sltsingleError: '',
            sltdoubleError: ''
        },
        // 输入框的弹窗
        noticInfo: {
            iptNoticInfo: {
                noticFlag: true,
                noticType: 'modal',
                noticModal: {
                    modalTitle: '标题',
                    modalContent: 'modal的一些个文案'
                }
            }
        },
        // 存放页面中所有渲染的数据
        options: {
            // 单选框
            radiosValue: [
                {
                    name: '单选一'
                },
                {
                    name: '单选二'
                },
                {
                    name: '单选三'
                },
                {
                    name: '不含以上情况'
                }
            ],
            // 复选框
            checkboxsValue: [
                {
                    name: '多选一'
                },
                {
                    name: '多选二'
                },
                {
                    name: '多选三'
                }
            ]
        },
        // 状态页配置数据
        errorConfig: {
            noNetwork: {
                icon: 'wifi',
                title: '网络不给力，请稍后重试',
                showBtn: true
            },
            warning: {
                icon: 'warning',
                title: '服务器开小差，请稍后重试',
                showBtn: true
            },
            noData: {
                icon: 'content',
                title: '未查询到相关信息',
                desc: '请核实您的查询条件',
                showBtn: false
            }
        }
    },

    /**
     * 页面加载时触发
     */
    onLoad() {
        this.setData({
            // 当前日期
            endDate: this.endDate(),
            // 开始日期
            startDate: '1950-1-1',
            // 测试数据，真实环境依赖 server 返回数据
            options: {...this.data.options, ...mockData.data}
        });
    },

    /**
     * 检测网络
     */
    check() {
        this.setData('isPageLoading', true);
        checkNetwork().then(res => {
            this.getDetail();
        })
        .catch(err => {
            this.setData({
                isPageLoading: false,
                pageResult: 'noNetwork'
            });
        });
    },

    /**
     * 发送请求
     *
     * @param {Object=} data 请求接口参数
     */
    getDetail(data = {}) {
        // 【需替换】：获取内容详情所需要的数据，请修改 url 字段为真实的请求地址，该接口仅做示例
        let params = {
            url: 'https://www.ceshi.com',
            method: 'GET',
            data,
            success: res => {
                // 接口正常返回处理逻辑
                if (+res.code === 0) {
                    if (Object.keys(res.data).length) {
                        this.setData({
                            options: {...this.data.options, ...res.data}
                        }, () => {
                            this.setData({
                                isPageLoading: false,
                                pageResult: ''
                            });
                        });
                    }
                    else {
                        // 没有数据
                        this.setData({
                            isPageLoading: false,
                            pageResult: 'noData'
                        });
                    }
                }
                else {
                    // 接口异常处理逻辑
                    this.setData({
                        isPageLoading: false,
                        pageResult: 'warning'
                    });
                }
            },
            fail: err => {
                // 接口异常处理逻辑
                this.setData({
                    isPageLoading: false,
                    pageResult: 'warning'
                });
            }
        };
        swan.request(params);
    },

    /**
     * input 清除事件
     *
     * @param {Event} e 事件对象
     * @param {string} e.iptname input的name
     * @param {string} e.iptvalue 获取value值
     */
    iptKeyClean(e) {
        this.setData(`formData.${e.iptname}`, e.iptvalue);
    },

    /**
     * input 输入事件
     *
     * @param {Event} e 事件对象
     * @param {string} e.iptname input的name
     * @param {string} e.iptvalue 获取value值
     */
    iptKeyInput(e) {
        this.setData(`formData.${e.iptname}`, e.iptvalue);
    },

    /**
     * input 身份证失焦事件
     *
     * @param {Event} e 事件对象
     * @param {string} e.iptvalue 获取value值
     */
    iptblursfz(e) {
        this.setData({
            'errorInfo.idcardError': checkIdcard(e.iptvalue)
        });
    },

    /**
     * input 手机号失焦事件
     *
     * @param {Event} e 事件对象
     * @param {string} e.iptvalue 获取value值
     */
    iptblursjh(e) {
        this.setData({
            'errorInfo.phoneError': checkPhone(e.iptvalue)
        });
    },

    /**
     * 日期 picker事件
     *
     * @param {Event} e 事件对象
     * @param {string} e.detail.detail.value 获取选中value值
     */
    handleChange1(e) {
        this.setData({
            'formData.date': e.detail.detail.value,
            'errorInfo.dateError': false
        });
    },

    /**
     * 获取位置成功
     *
     * @param {Object} res 位置信息
     * @param {string} res.address 获取地方名
     */
    choosesuccess(res) {
        this.setData({
            'formData.place': res.address,
            'errorInfo.placeError': false
        });
    },

    /**
     * 获取位置失败
     */
    choosefail() {
        this.setData({
            'formData.place': '',
            'errorInfo.placeError': true
        });
    },

    /**
     * 选择框改变 事件
     *
     * @param {Event} e 事件对象
     * @param {string} e.detail.detail.value 获取选中值
     */
    handleChange3(e) {
        this.setData({
            'formData.sltsingle': e.detail.detail.value,
            'errorInfo.sltsingleError': false
        });
        /**
        picker 组件中 mode 不同 value 的数据类型不同
            mode === region
                e.detail.detail.value === ['陕西省','市','县']
            mode === selector
                e.detail.detail.value, === [0,1];
            mode === date/time
                e.detail.detail.value,=== YYYY-MM-DD/hh:mm
        */
    },

    /**
     * 联动 cascader-picker 事件
     *
     * @param {Event} e 事件对象
     * @param {string} e.value 获取选中值
     */
    cacadaChange(e) {
        // value数据格式：e.value == [0,0,0] 索引
        this.setData('formData.sltdouble', e.value);
        this.setData('errorInfo.sltdoubleError', false);
    },

    /**
     * 单选框改变事件
     *
     * @param {Event} e 事件对象
     * @param {string} e.detail 获取 value 值
     */
    radioChange(e) {
        this.setData('formData.radios', e.detail);
    },

    /**
     * 多选框改变事件
     *
     * @param {Event} e 事件对象
     * @param {string} e.detail 获取 value 值
     */
    checkboxChange(e) {
        this.setData({
            'formData.checkbox': e.detail,
            'formData.nochecked': false
        });
    },

    /**
     * 多选不不包含以上情况
     *
     * @param {Event} e 事件对象
     * @param {string} e.detail 获取value值
     */
    noHas(e) {
        this.setData({
            'formData.nochecked': e.detail,
            'formData.checkbox': []
        });
    },

    /**
     * 多文本输入事件
     *
     * @param {Event} e 事件对象
     * @param {string} e.value 获取多文本 value 值
     */
    iptTextarea(e) {
        this.setData('formData.textBox', e.value);
    },

    /**
     * 删除图片
     *
     * @param {Event} e 事件对象
     * @param {Array} e.detail.path 上传的所有图片
     */
    clickDelete(e) {
        this.setData('formData.uploadphotos', e.detail.path);
    },

    /**
     * 上传成功
     *
     * @param {Object} res 返回数据
     * @param {Array} res.res.data 上传的所有图片
     */
    uploadsuccess(res) {
        this.data.formData.uploadphotos.push(res.res.data);
        this.setData('formData.uploadphotos', this.data.formData.uploadphotos);
    },

    /**
     * 上传url为空
     */
    urlempty() {
        this.showToast('调用成功，但依赖第三方的sever接口实现上传功能');
    },

    /**
     * 预览图片失败
     */
    previewfail() {
        this.showToast('调用成功，但依赖第三方的sever接口返回图片线上地址');
    },

    /**
     * 温馨提示链接，开发者可自定义需要跳转的页面
     */
    toView() {
        this.showToast('可自定义需要跳转的页面或操作');
        swan.redirectTo({
            url: ''
        });
    },

    /**
     * 点击上一步按钮触发事件
     */
    prev() {
        this.setData({
            active: this.data.active-- <= 0 ? 0 : this.data.active
        });
    },

    /**
     * 点击下一步按钮触发事件
     */
    next() {
        // 输入框，选择框根据错误状态 判断 toast 的状态,
        // 多选，单选，长文本，上传照片根据 value 值 判断 toast 的状态
        const formData = this.data.formData;
        let datas = '';
        switch (this.data.active) {
            case 1:
                this.verifyfirst();
                break;
            case 2:
                if (Number(formData.radios) < 0) {
                    this.showToast('请选择单选');
                    return;
                }
                if (!formData.checkbox.length && !formData.nochecked) {
                    this.showToast('请选择多选');
                    return;
                }
                this.setData('active', 3);
                break;
            case 3:
                if (!formData.textBox) {
                    this.showToast('请输入长文本');
                    return;
                }
                this.setData('active', 4);
                this.fiveInfor();
                break;
            case 4:
                // 设置参数
                datas = JSON.stringify(this.parameter());
                swan.showModal({
                    title: '信息确认信息',
                    content: '请确认填写的信息无误，提交后不支持修改',
                    confirmText: '确认提交',
                    confirmColor: '#108EE9',
                    cancelText: '返回修改',
                    cancelColor: '#999',
                    success(res) {
                        if (res.confirm) {
                            swan.redirectTo({
                                url: `./result/result?result=${datas}`
                            });
                        }
                    }
                });
                break;
        }
    },

    /**
     * 第一步的校验函数，错误飘红，并 toast 提示相关错误信息
     */
    verifyfirst() {
        const formData = this.data.formData;
        // 页面一中 input,选择框的 error 状态设置
        let errorInfo = {
            idcardError: checkIdcard(formData.idcard),
            phoneError: checkPhone(formData.phone),
            dateError: !formData.date,
            placeError: !formData.place,
            iptoneError: checkEmpty(formData.iptone, '请输入输入框一'),
            ipttwoError: checkEmpty(formData.ipttwo, '请输入输入框二'),
            sltsingleError: formData.sltsingle < 0,
            sltdoubleError: formData.sltdouble.length !== 3
        };
        this.setData({errorInfo}, () => {
            this.showCurToast();
        });
    },

    /**
     * 页面一的 toast 弹窗
     */
    showCurToast() {
        const errorInfo = this.data.errorInfo;
        if (errorInfo.idcardError.errFlag) {
            this.showToast('请输入正确的18位身份证号');
            return;
        }
        if (errorInfo.phoneError.errFlag) {
            this.showToast('请输入正确的11位手机号');
            return;
        }
        if (errorInfo.dateError) {
            this.showToast('请选择');
            return;
        }
        if (errorInfo.placeError) {
            this.showToast('请选择');
            return;
        }
        if (errorInfo.iptoneError.errFlag) {
            this.showToast('请输入一');
            return;
        }
        if (errorInfo.ipttwoError.errFlag) {
            this.showToast('请输入二');
            return;
        }
        if (errorInfo.sltsingleError) {
            this.showToast('请选择选择框一');
            return;
        }
        if (errorInfo.sltdoubleError) {
            this.showToast('请选择联动');
            return;
        }
        this.setData('active', 2);
    },

    /**
     * 核对信息页面的数据
     */
    fiveInfor() {
        let parameter = this.parameter();
        let informateFour = {
            lists: [
                {
                    bigTitle: '标题一',
                    msg: [
                        {
                            title: '身份证号',
                            sub: parameter.idcard
                        },
                        {
                            title: '手机号',
                            sub: parameter.phone
                        },
                        {
                            title: '日期',
                            sub: parameter.date
                        },
                        {
                            title: '所在位置',
                            sub: parameter.place
                        }
                    ]
                },
                {
                    bigTitle: '标题二',
                    msg: [
                        {
                            title: '输入框',
                            sub: parameter.iptone
                        },
                        {
                            title: '输入框',
                            sub: parameter.ipttwo
                        },
                        {
                            title: '选择框',
                            sub: parameter.sltsingle
                        },
                        {
                            title: '选择框',
                            sub: parameter.sltdouble
                        }
                    ]
                }
            ],
            expand: [
                {
                    bigTitle: '多选标题',
                    msg: parameter.checkbox
                },
                {
                    bigTitle: '单选标题',
                    msg: parameter.radios
                },
                {
                    bigTitle: '长问本输入框标题',
                    msg: parameter.textBox
                }
            ],
            photos: {
                bigTitle: '上传照片标题',
                msg: parameter.uploadphotos
            }
        };
        this.setData({informateFour});
    },

    /**
     *  toast弹窗
     *
     * @param {string} msg toast 标题
     */
    showToast(msg) {
        swan.showToast({
            title: msg,
            icon: 'none'
        });
    },

    /**
     * formData中的信息整理
     *
     * @return {Object} 返回页面四的数据
     */
    parameter() {
        const {
            options,
            formData
        } = this.data;
        // 联动picker， 模板中找到联动 value[0,0,0] 对应的数据，然后拼接
        let arr = [];
        formData.sltdouble.forEach((item, index) => {
            arr.push(options.cascader[index][item].name);
        });

        // 多选框数据 索引排序，找出对应数据，拼接成字符传
        let datas = options.checkboxsValue;
        // 多选框 [0,1]/(选中索引) => ['多选一','多选二']/(选中索引对的值) => '多选一，多选二'/(字符串)
        let checkboxs = formData.checkbox.sort((a, b) => a - b).map(item => datas[item].name).join('、');
        return {
            ...formData,
            sltdouble: arr.join('、'),
            radios: options.radiosValue[formData.radios].name,
            checkbox: formData.nochecked ? '不含以上情况' : checkboxs,
            sltsingle: options.singPicker[formData.sltsingle].name
        };
    },

    /**
     * 结束日期
     *
     * @return {string} 当前日期
     */
    endDate() {
        let date = new Date();
        let dateYear = date.getFullYear();
        let dateMonth = date.getMonth() + 1;
        let dateDate = date.getDate();
        return `${dateYear}-${dateMonth}-${dateDate}`;
    },

    /**
     * 自定义页面返回按钮的事件
     */
    navigate() {
        // 自定义 navbar 组件业务方跳转逻辑
        swan.switchTab({
            url: '/entry/extensions/extensions?data=serviceSupport'
        });
    }
});
