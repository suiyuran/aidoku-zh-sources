# aidoku-zh-sources

[繁體中文](README.md) | 简体中文

Aidoku 中文图源

## 如何使用

推荐使用 [v2 格式](https://aidoku.app/add-source-list/?url=https://raw.githubusercontent.com/NullKeeper-dev/aidoku-zh-sources/main/public/v2/index.min.json) 添加图源，不会出现「legacy source list」警告。

旧版 Aidoku 仍可使用 [legacy 地址](https://aidoku.app/add-source-list/?url=https://raw.githubusercontent.com/NullKeeper-dev/aidoku-zh-sources/main/public/)；原有目录地址和 `public/index.json` 均继续保留。

如果 GitHub Raw 无法访问，可以使用 [jsDelivr legacy 地址](https://aidoku.app/add-source-list/?url=https://cdn.jsdelivr.net/gh/NullKeeper-dev/aidoku-zh-sources@main/public/)。

## 图源列表

- [18漫画](https://18mh.org)
- [包子漫画1](https://www.baozimh.com)
- [包子漫画2](https://baozimh.org)
- [香香腐宅](https://boylove.cc)
- [动漫屋](https://www.dm5.com)
- [G站漫画](https://godamh.com)
- [HO5HO](https://www.ho5ho.com)
- [Komiic漫画](https://komiic.com)
- [Māngabz](https://mangabz.com)
- [漫画柜（Manhuagui）](https://www.manhuagui.com)
- [漫客栈](https://www.mkzhan.com)
- [漫小肆韩漫](https://www.mxshm.top)
- [热辣漫画](https://www.manga2026.com)
- [绅士漫画](https://www.wn07.cfd)
- [再漫画](https://www.zaimanhua.com)
- [zero搬运网](https://www.zerobyw33.com)

以上 16 个图源都已完成“列表 → 详情 → 章节 → 图片”测试。包子漫画网站会把部分最新章节替换成“请使用 App”宣传图；来源会跳过这类假页面，较早的可读章节仍可正常使用。

包子漫画2和 G站已改用两站当前共用的 `v2.apikk.top` API，并按章节的 `line` 选择图片主机。两个来源的设置内可切换 Cloudflare、国际备用和旧版兼容三条 CDN 线路。全球分布式测试的台湾、新加坡、德国和多个美国节点都能读取图片；如果 Comcast/Xfinity 将全部线路导向 SafeBrowse，需要在 xFi Advanced Security 中允许该域名或改用未拦截的网络，来源端无法绕过 ISP 过滤。

## 暂时下架

以下图源仍保留源码，待站点恢复或完成较大改版后再重新发布：

- ~~[139漫画](https://139mh.com)~~：JavaScript 防机器人页
- ~~[巴卡漫画](https://bakamh.com)~~：阅读页持续返回 HTTP 403
- ~~[哔哩漫画](https://www.bilimanga.net)~~：图片 CDN 持续返回 HTTP 403
- ~~[拷贝漫画](https://www.mangacopy.com)~~：新版章节 API 返回空图片组
- ~~[漫画狗](https://dogemanga.com)~~：美国线路搜索／列表端点被 Cloudflare 返回 HTTP 403；旧包缺失的图标已修复
- ~~[古风漫画](https://www.gufengmh.com)~~：站点返回 HTTP 403 / TLS 异常
- ~~[嗨皮漫画](https://m.happymh.com)~~：持续出现人机验证
- ~~[哔咔漫画](https://manhuabika.com)~~：必须登录，无法进行匿名验收
- ~~[漫画DB](https://www.manhuadb.com)~~：连接超时或 JavaScript 挑战
- ~~[漫画人](https://www.manhuaren.com)~~：旧签名失效，需重写设备认证
- ~~[MYCOMIC](https://mycomic.com/cn)~~：Cloudflare HTTP 403
- ~~[Nico](https://nicohub.cc/index?category=comic)~~：JavaScript 防机器人页
- ~~[NoyAcg](https://noy1.top)~~：新版 API 强制登录
- ~~[韩漫库](https://se8.us)~~：首页 HTTP 403，漫画路由 HTTP 404
- ~~[vomic漫画](https://www.vomicmh.com)~~：站点已改版，匿名章节页强制登录且不返回图片数据
- ~~[野蛮漫画](https://yemancomic.com)~~：美国线路持续返回 HTTP 403

## 已移除

以下站点已确认永久失效或不再提供原漫画服务：无敌漫画、动漫狂、Hanime1 漫画区、ISEKAI、喵上漫画、言耽社、旧版 zero搬运网（`zerobyw.github.io`）。其源码及未被索引引用的旧包已在本次整理中删除；Git 历史仍可供追溯。

## 状态

2026-07-27 在美国（Comcast/Xfinity、SJC 出口）重新审计。新增香香腐宅、动漫屋、Māngabz、漫画柜、热辣漫画与新版 zero搬运网；修复包子漫画 App 宣传假页、包子漫画2／G站共用 API、章节解码与多 CDN 线路、香香腐宅新章节格式与 13 栏图片重排、漫小肆／绅士漫画新域名，以及所有 legacy 包的大小写兼容图标（包含漫画狗）。G站不是封锁亚洲以外访客；它与包子漫画2在全球抽测节点可读，只有部分 ISP 的安全过滤仍可能拦截图片域名。

当前发布 16 个通过构建和完整阅读链测试的图源；仓库保留 32 个仍可维护的来源，永久失效的 7 个来源及 74 个未被索引引用的旧包／图标已清理。候选来源也交叉检查了 Aidoku Community 的 21 个中文来源与 Keiyoushi 的 51 个中文扩展；只有可匿名访问、存在 Aidoku 实现且图片能在实际或分布式美国节点下载的来源才会加入。完整逐源结果、三部作品验证、候选排除原因及相关 issue 见 [AUDIT.md](AUDIT.md)。
