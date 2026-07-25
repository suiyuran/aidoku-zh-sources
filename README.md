# aidoku-zh-sources

Aidoku 中文图源

## 如何使用

推荐使用 [v2 格式](https://aidoku.app/add-source-list/?url=https://raw.githubusercontent.com/NullKeeper-dev/aidoku-zh-sources/main/public/v2/index.min.json) 添加图源，不会出现「legacy source list」警告。

旧版 Aidoku 仍可使用 [legacy 地址](https://aidoku.app/add-source-list/?url=https://raw.githubusercontent.com/NullKeeper-dev/aidoku-zh-sources/main/public/)；原有目录地址和 `public/index.json` 均继续保留。

如果 GitHub Raw 无法访问，可以使用 [jsDelivr legacy 地址](https://aidoku.app/add-source-list/?url=https://cdn.jsdelivr.net/gh/NullKeeper-dev/aidoku-zh-sources@main/public/)。

## 图源列表

- [18漫画](https://18mh.org)
- [包子漫画1](https://www.baozimh.com)
- [包子漫画2](https://baozimh.org)
- [漫画狗](https://dogemanga.com)
- [G站漫画](https://godamh.com)
- [HO5HO](https://www.ho5ho.com)
- [Komiic漫画](https://komiic.com)
- [漫客栈](https://www.mkzhan.com)
- [漫小肆韩漫](https://www.mxs13.cc)
- [绅士漫画](https://www.wnacg.com)
- [野蛮漫画](https://yemancomic.com)
- [再漫画](https://www.zaimanhua.com)

## 暂时下架

以下图源仍保留源码，待站点恢复或完成较大改版后再重新发布：

- ~~[139漫画](https://139mh.com)~~：JavaScript 防机器人页
- ~~[巴卡漫画](https://bakamh.com)~~：阅读页持续返回 HTTP 403
- ~~[哔哩漫画](https://www.bilimanga.net)~~：图片 CDN 持续返回 HTTP 403
- ~~[拷贝漫画](https://www.mangacopy.com)~~：新版章节 API 返回空图片组
- ~~[古风漫画](https://www.gufengmh.com)~~：站点返回 HTTP 403 / TLS 异常
- ~~[嗨皮漫画](https://m.happymh.com)~~：持续出现人机验证
- ~~[哔咔漫画](https://manhuabika.com)~~：必须登录，无法进行匿名验收
- ~~[漫画DB](https://www.manhuadb.com)~~：连接超时或 JavaScript 挑战
- ~~[漫画人](https://www.manhuaren.com)~~：旧签名失效，需重写设备认证
- ~~[MYCOMIC](https://mycomic.com/cn)~~：Cloudflare HTTP 403
- ~~[Nico](https://nicohub.cc/index?category=comic)~~：JavaScript 防机器人页
- ~~[NoyAcg](https://noy1.top)~~：新版 API 强制登录
- ~~[韩漫库](https://se8.us)~~：首页 HTTP 403，漫画路由 HTTP 404
- ~~[vomic漫画](https://www.vomicmh.com)~~：站点已改版，旧 API 返回 HTTP 404

## 已移除

以下站点已确认永久失效或不再提供原漫画服务：无敌漫画、动漫狂、Hanime1 漫画区、ISEKAI、喵上漫画、言耽社、zero搬运网。

## 状态

2026-07-26 完成全量审计：修复 G站漫画、包子漫画、漫画狗、HO5HO、漫小肆、绅士漫画等站点变更，并迁移至 Aidoku v2 图源列表格式。当前发布 12 个通过构建和在线阅读链路测试的图源；其余图源已按上面的原因下架或移除。完整的逐源结果、测试标题和相关 issue 见 [AUDIT.md](AUDIT.md)。
