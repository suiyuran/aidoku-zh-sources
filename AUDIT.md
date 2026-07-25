# 图源审计记录

审计日期：2026-07-26（Asia/Taipei）

本次审计枚举了 `src/as/` 和 `src/rust/` 下的全部 33 个图源。构建结果表示源码能够生成 `.aix`；在线结果依次检查列表、漫画详情、章节列表、章节页面及至少一张原图。图标同时检查为包内存在且可读取。被标记为“暂时下架”或“已移除”的包不会进入当前 v2/legacy 索引，但源码仍保留以便后续维护。

## 汇总

| 状态 | 数量 | 说明 |
| --- | ---: | --- |
| 发布 | 12 | 构建和完整在线阅读链路均成功 |
| 暂时下架 | 14 | 反爬、认证、API 或站点改版问题，仍有恢复可能 |
| 已移除 | 7 | 域名失效、停站、停服或原漫画服务已不存在 |
| 合计 | 33 | 全部图源构建成功 |

## 逐源结果

| 图源 | 基础 URL | 构建 | 在线检查 | 失败类型 / 备注 | 相关 issue |
| --- | --- | :---: | --- | --- | --- |
| `zh.139mh` 139漫画 | <https://139mh.com> | ✅ | ❌ 列表失败 | HTTP 200，但仅返回很小的 JavaScript “Loading...” 防机器人页；API 同样不可用 | — |
| `zh.18mh` 18漫画 | <https://18mh.org> | ✅ | ✅ 完整链路 | 列表 18 项；测试漫画 42 章，章节 40 页，图片 HTTP 200 | — |
| `zh.55dmh` 无敌漫画 | <https://www.55dmh.com> | ✅ | ❌ 首页失败 | 连接被拒绝；站点确认停止服务 | — |
| `zh.bakamh` 巴卡漫画 | <https://bakamh.com> | ✅ | ❌ 章节页面失败 | 列表需预热 Cookie/Client Hints，阅读路由仍持续 HTTP 403 | [#124](https://github.com/suiyuran/aidoku-zh-sources/issues/124)、[#148](https://github.com/suiyuran/aidoku-zh-sources/issues/148) |
| `zh.baozimh1` 包子漫画1 | <https://www.baozimh.com> | ✅ | ✅ 3 个标题完整链路 | 改用当前 App 阅读页，直接取得原始 CDN 图片，见下方专项结果 | [#138](https://github.com/suiyuran/aidoku-zh-sources/issues/138)、[Skittyblock #485](https://github.com/Skittyblock/aidoku-community-sources/issues/485)、[Suwayomi #686](https://github.com/Suwayomi/Suwayomi-Server/issues/686) |
| `zh.baozimh2` 包子漫画2 | <https://baozimh.org> | ✅ | ✅ 3 个标题完整链路 | 更新至当前 v3 搜索/章节后端和站点解码器，见下方专项结果 | [#138](https://github.com/suiyuran/aidoku-zh-sources/issues/138) |
| `zh.bilicomic` 哔哩漫画 | <https://www.bilimanga.net> | ✅ | ❌ 图片失败 | 列表、详情和章节页面可解析；`i.motiezw.com` 图片 CDN 持续 HTTP 403 | — |
| `zh.cartoonmad` 动漫狂 | <https://www.cartoonmad.com> | ✅ | ❌ 首页失败 | HTTP 200，但所有路径均返回 114 字节的 `/lander` 跳转脚本；域名已停放 | — |
| `zh.copymanga` 拷贝漫画 | <https://www.mangacopy.com> | ✅ | ❌ 章节图片失败 | 旧客户端版本被拒；切换当前参数和 `ccz` 解密后，多个标题仍返回空图片组 | [#128](https://github.com/suiyuran/aidoku-zh-sources/issues/128)、[#135](https://github.com/suiyuran/aidoku-zh-sources/issues/135) |
| `zh.dogemanga` 漫画狗 | <https://dogemanga.com> | ✅ | ✅ 完整链路 | 修复空查询参数和封面选择器；测试《王者天下》655 章，章节 19 张图，图片 HTTP 200 | — |
| `zh.godamanga` G站漫画 | <https://godamh.com> | ✅ | ✅ 3 个标题完整链路 | 旧章节接口明确返回 HTTP 410；已更新后端和解码器，见下方专项结果 | [#142](https://github.com/suiyuran/aidoku-zh-sources/issues/142)、[#144](https://github.com/suiyuran/aidoku-zh-sources/issues/144) |
| `zh.gufengmh` 古风漫画 | <https://www.gufengmh.com> | ✅ | ❌ 首页失败 | HTTPS/TLS 异常，HTTP 路径返回 146 字节 nginx HTTP 403 | — |
| `zh.hanime1` Hanime1.me | <https://hanime1.me/comics> | ✅ | ❌ 列表失败 | 漫画路由在移动端 HTTP 404，桌面端 Cloudflare HTTP 403；主站仅剩视频服务 | [#147](https://github.com/suiyuran/aidoku-zh-sources/issues/147) |
| `zh.happymh` 嗨皮漫画 | <https://m.happymh.com> | ✅ | ❌ 首页/API 失败 | HTTP 403 人机验证页；当前图片算法也已改为新的加密扫描格式 | — |
| `zh.ho5ho` HO5HO | <https://www.ho5ho.com> | ✅ | ✅ 完整链路 | 更新 v2 卡片、详情和阅读页选择器；测试漫画 1 章 3 页，图片 HTTP 200 | — |
| `zh.isekai` ISEKAI | <https://isekai.ch> | ✅ | ❌ 首页失败 | 所有路径均为约 980 字节的域名停放页 | — |
| `zh.komiic` Komiic漫画 | <https://komiic.com> | ✅ | ✅ 完整链路 | GraphQL 列表/详情/约 214 章、章节页及图片均成功 | [#127](https://github.com/suiyuran/aidoku-zh-sources/issues/127) |
| `zh.manhuabika` 哔咔漫画 | <https://manhuabika.com> | ✅ | ⚠️ 无法匿名验收 | 服务必须使用有效账号登录；本次审计不创建账号，也没有可用测试凭据 | — |
| `zh.manhuadb` 漫画DB | <https://www.manhuadb.com> | ✅ | ❌ 首页失败 | HTTPS 连接超时；HTTP 返回混淆 JavaScript 挑战 | — |
| `zh.manhuaren` 漫画人 | <https://www.manhuaren.com> | ✅ | ❌ API 认证失败 | 静态 token/签名已失效；当前服务需要匿名设备注册、RSA 和新签名流程 | — |
| `zh.miaoshangmanhua` 喵上漫画 | <https://www.miaoshangmanhua.cc> | ✅ | ❌ DNS 失败 | 域名无法解析 | — |
| `zh.mkzhan` 漫客栈 | <https://www.mkzhan.com> | ✅ | ✅ 完整链路 | 列表 API、详情、章节、页面和图片均成功 | — |
| `zh.msxhm` 漫小肆韩漫 | <https://www.mxs13.cc> | ✅ | ✅ 完整链路 | 更新域名并修复空查询参数；测试漫画 3 章，章节 74 页，图片 HTTP 200 | — |
| `zh.mycomic` MYCOMIC | <https://mycomic.com/cn> | ✅ | ❌ 首页失败 | Cloudflare HTTP 403 | — |
| `zh.nicohub` Nico | <https://nicohub.cc/index?category=comic> | ✅ | ❌ 首页失败 | HTTP 200，但只有约 492 字节的 JavaScript “Loading...” 挑战页 | — |
| `zh.noy1` NoyAcg | <https://noy1.top> | ✅ | ❌ API 认证失败 | 旧 `/api/booklist_v2` 与 beta `/api/b1`、`/api/v4` 均返回 `status: login` | [#151](https://github.com/suiyuran/aidoku-zh-sources/issues/151) |
| `zh.se8` 韩漫库 | <https://se8.us> | ✅ | ❌ 首页/路由失败 | 首页 HTTP 403 nginx；旧漫画路由和 `index.php` HTTP 404 | — |
| `zh.vomicmh` vomic漫画 | <https://www.vomicmh.com> | ✅ | ❌ API 失败 | 网站仍在线但已重写为新前端；`api.vomicmh.com` 旧接口 HTTP 404 | — |
| `zh.wnacg` 绅士漫画 | <https://www.wnacg.com> | ✅ | ✅ 完整链路 | 更新有效域名；列表 20 项、测试漫画 1 章 188 页，图片 HTTP 200 | [#149](https://github.com/suiyuran/aidoku-zh-sources/issues/149)、[#152](https://github.com/suiyuran/aidoku-zh-sources/issues/152) |
| `zh.yandanshe` 言耽社 | <https://yandanshe.com> | ✅ | ❌ DNS 失败 | 域名无法解析 | — |
| `zh.yemancomic` 野蛮漫画 | <https://yemancomic.com> | ✅ | ✅ 完整链路 | 列表 48 项、测试漫画 15 章、章节 66 页，图片 HTTP 200 | — |
| `zh.zaimanhua` 再漫画 | <https://www.zaimanhua.com> | ✅ | ✅ 完整链路 | 列表、详情、10 章、章节页和图片 HTTP 200 | — |
| `zh.zerobywns` zero搬运网 | <https://zerobyw.github.io> | ✅ | ❌ 服务失效 | GitHub Pages 只剩说明页；默认内容域名已变成无关 WordPress，旧插件接口 HTTP 404 | [#145](https://github.com/suiyuran/aidoku-zh-sources/issues/145) |

## G站漫画专项验证

`godamh` 已从停止服务的旧章节接口迁移到当前 `api-get-v3.mgsearcher.com/api/v2/chapter/getinfo`，并按站点 `assets/runtime/chapter-decoder.js` 的现行逻辑实现响应解码。

| 测试标题 | 章节数 | 抽测章节页数 | 图片结果 |
| --- | ---: | ---: | --- |
| 武炼巅峰 | 3875 | 75 | HTTP 200，WebP，774458 字节 |
| 我独自升级 | 210 | 36 | HTTP 200，19132 字节 |
| 大王饶命 | 956 | 110 | HTTP 200，80912 字节 |

三个标题均通过列表、详情、章节列表、章节页面解码和原图下载。

## 包子漫画专项验证

`baozimh1` 不再对旧阅读页的合成图片做固定尺寸裁剪，而是使用当前 `appcn.baozimh.com/baozimhapp` 阅读页中的 `.comic-contain__item` 原始图片地址。该方式避开了会随横幅尺寸变化而失效的旧水印裁剪逻辑。`baozimh2` 使用与当前网页一致的 v3 后端和章节解码器。

包子漫画1：

| 测试标题 | 章节数 | 抽测章节页数 | 图片结果 |
| --- | ---: | ---: | --- |
| 武炼巅峰 | 3874 | 75 | HTTP 200，JPEG，746210 字节 |
| カコミスル老師四格合集 | 2710 | 3 | HTTP 200，262702 字节 |
| 中国惊奇先生 | 1828 | 48 | HTTP 200，1044126 字节 |

包子漫画2：

| 测试标题 | 章节数 | 抽测章节页数 | 图片结果 |
| --- | ---: | ---: | --- |
| 武炼巅峰 | 3875 | 75 | HTTP 200 |
| 我独自升级 | 210 | 36 | HTTP 200 |
| 大王饶命 | 956 | 110 | HTTP 200 |

两个图源的三个测试标题均通过列表、详情、章节列表、章节页面和原图下载。

## 索引兼容性

- `public/v2/index.min.json`：Aidoku 当前 v2 对象格式，结构为 `name` + `sources`，由当前官方 CLI 生成并保持 canonical minified 输出。
- `public/v2/index.json`：同一 v2 索引的可读版本。
- `public/index.json`：保留的 legacy 数组格式。
- `public/index.min.json`：保留的 legacy 压缩数组。Aidoku 对目录型旧订阅会尝试此文件，因此不能用 v2 对象覆盖它。

当前构建只把通过完整在线链路测试的 12 个图源写入两套索引；旧版本 `.aix` 文件仍保留，避免已有安装的下载地址突然失效。

本仓库的解析器仍使用 Aidoku 旧版 WASM 源 API，这与“图源列表 v2 schema”是两个不同层级。当前 Aidoku 的 `SourceManager` 会在新 runner 初始化失败后回退到 `LegacySourceRunner`，因此这些包仍可安装和运行。为兼容两条加载路径，构建包同时包含当前 CLI 使用的 `Payload/icon.png` 和 legacy runner 使用的 `Payload/Icon.png`。官方 CLI 已确认全部发布包的 `source.json` 合法、图标为 128×128；其完整 `verify` 仍会提示旧 WASM exports / `filters.json`，直到未来逐源迁移源 API。
