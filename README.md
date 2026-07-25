# aidoku-zh-sources

繁體中文 | [简体中文](README.zh-CN.md)

Aidoku 中文圖源

## 如何使用

推薦使用 [v2 格式](https://aidoku.app/add-source-list/?url=https://raw.githubusercontent.com/NullKeeper-dev/aidoku-zh-sources/main/public/v2/index.min.json) 新增圖源，不會出現「legacy source list」警告。

舊版 Aidoku 仍可使用 [legacy 位址](https://aidoku.app/add-source-list/?url=https://raw.githubusercontent.com/NullKeeper-dev/aidoku-zh-sources/main/public/)；原有目錄位址和 `public/index.json` 均繼續保留。

如果無法存取 GitHub Raw，可以使用 [jsDelivr legacy 位址](https://aidoku.app/add-source-list/?url=https://cdn.jsdelivr.net/gh/NullKeeper-dev/aidoku-zh-sources@main/public/)。

## 圖源列表

- [18漫畫](https://18mh.org)
- [包子漫畫1](https://www.baozimh.com)
- [包子漫畫2](https://baozimh.org)
- [漫畫狗](https://dogemanga.com)
- [G站漫畫](https://godamh.com)
- [HO5HO](https://www.ho5ho.com)
- [Komiic漫畫](https://komiic.com)
- [漫客棧](https://www.mkzhan.com)
- [漫小肆韓漫](https://www.mxs13.cc)
- [紳士漫畫](https://www.wnacg.com)
- [野蠻漫畫](https://yemancomic.com)
- [再漫畫](https://www.zaimanhua.com)

## 暫時下架

以下圖源仍保留原始碼，待網站恢復或完成較大改版後再重新發布：

- ~~[139漫畫](https://139mh.com)~~：JavaScript 防機器人頁
- ~~[巴卡漫畫](https://bakamh.com)~~：閱讀頁持續返回 HTTP 403
- ~~[嗶哩漫畫](https://www.bilimanga.net)~~：圖片 CDN 持續返回 HTTP 403
- ~~[拷貝漫畫](https://www.mangacopy.com)~~：新版章節 API 返回空圖片組
- ~~[古風漫畫](https://www.gufengmh.com)~~：網站返回 HTTP 403 / TLS 異常
- ~~[嗨皮漫畫](https://m.happymh.com)~~：持續出現人機驗證
- ~~[嗶咔漫畫](https://manhuabika.com)~~：必須登入，無法進行匿名驗收
- ~~[漫畫DB](https://www.manhuadb.com)~~：連線逾時或 JavaScript 挑戰
- ~~[漫畫人](https://www.manhuaren.com)~~：舊簽章失效，需重寫裝置驗證
- ~~[MYCOMIC](https://mycomic.com/cn)~~：Cloudflare HTTP 403
- ~~[Nico](https://nicohub.cc/index?category=comic)~~：JavaScript 防機器人頁
- ~~[NoyAcg](https://noy1.top)~~：新版 API 強制登入
- ~~[韓漫庫](https://se8.us)~~：首頁 HTTP 403，漫畫路由 HTTP 404
- ~~[vomic漫畫](https://www.vomicmh.com)~~：網站已改版，舊 API 返回 HTTP 404

## 已移除

以下網站已確認永久失效或不再提供原漫畫服務：無敵漫畫、動漫狂、Hanime1 漫畫區、ISEKAI、喵上漫畫、言耽社、zero搬運網。

## 狀態

2026-07-26 完成完整稽核：修復 G站漫畫、包子漫畫、漫畫狗、HO5HO、漫小肆、紳士漫畫等網站變更，並遷移至 Aidoku v2 圖源列表格式。目前發布 12 個通過建置和線上閱讀流程測試的圖源；其餘圖源已依上述原因下架或移除。完整的逐源結果、測試作品和相關 issue 請參閱 [AUDIT.md](AUDIT.md)。
