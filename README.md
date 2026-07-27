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
- [香香腐宅](https://boylove.cc)
- [動漫屋](https://www.dm5.com)
- [G站漫畫](https://godamh.com)
- [HO5HO](https://www.ho5ho.com)
- [Komiic漫畫](https://komiic.com)
- [Māngabz](https://mangabz.com)
- [漫畫櫃（Manhuagui）](https://www.manhuagui.com)
- [漫客棧](https://www.mkzhan.com)
- [漫小肆韓漫](https://www.mxshm.top)
- [熱辣漫畫](https://www.manga2026.com)
- [紳士漫畫](https://www.wn07.cfd)
- [再漫畫](https://www.zaimanhua.com)
- [zero搬運網](https://www.zerobyw33.com)

以上 16 個圖源都已完成「列表 → 詳情 → 章節 → 圖片」測試。包子漫畫網站會把部分最新章節替換成「請使用 App」宣傳圖；來源會略過這類假頁面，較早的可讀章節仍可正常使用。

包子漫畫2與 G站已改用兩站目前共用的 `v2.apikk.top` API，並依章節的 `line` 選擇圖片主機。兩個來源的設定內可切換 Cloudflare、國際備用與舊版相容三條 CDN 線路。全球分散式測試的臺灣、新加坡、德國及多個美國節點都能讀圖；若 Comcast/Xfinity 將全部線路導向 SafeBrowse，需在 xFi Advanced Security 允許該網域或改用未攔截的網路，來源端無法繞過 ISP 過濾。

## 暫時下架

以下圖源仍保留原始碼，待網站恢復或完成較大改版後再重新發布：

- ~~[139漫畫](https://139mh.com)~~：JavaScript 防機器人頁
- ~~[巴卡漫畫](https://bakamh.com)~~：閱讀頁持續返回 HTTP 403
- ~~[嗶哩漫畫](https://www.bilimanga.net)~~：圖片 CDN 持續返回 HTTP 403
- ~~[拷貝漫畫](https://www.mangacopy.com)~~：新版章節 API 返回空圖片組
- ~~[漫畫狗](https://dogemanga.com)~~：美國線路搜尋／列表端點被 Cloudflare 返回 HTTP 403；舊套件的缺失圖示已修復
- ~~[古風漫畫](https://www.gufengmh.com)~~：網站返回 HTTP 403 / TLS 異常
- ~~[嗨皮漫畫](https://m.happymh.com)~~：持續出現人機驗證
- ~~[嗶咔漫畫](https://manhuabika.com)~~：必須登入，無法進行匿名驗收
- ~~[漫畫DB](https://www.manhuadb.com)~~：連線逾時或 JavaScript 挑戰
- ~~[漫畫人](https://www.manhuaren.com)~~：舊簽章失效，需重寫裝置驗證
- ~~[MYCOMIC](https://mycomic.com/cn)~~：Cloudflare HTTP 403
- ~~[Nico](https://nicohub.cc/index?category=comic)~~：JavaScript 防機器人頁
- ~~[NoyAcg](https://noy1.top)~~：新版 API 強制登入
- ~~[韓漫庫](https://se8.us)~~：首頁 HTTP 403，漫畫路由 HTTP 404
- ~~[vomic漫畫](https://www.vomicmh.com)~~：網站已改版，匿名章節頁強制登入且不返回圖片資料
- ~~[野蠻漫畫](https://yemancomic.com)~~：美國線路持續返回 HTTP 403

## 已移除

以下網站已確認永久失效或不再提供原漫畫服務：無敵漫畫、動漫狂、Hanime1 漫畫區、ISEKAI、喵上漫畫、言耽社、舊版 zero搬運網（`zerobyw.github.io`）。其原始碼及未被索引引用的舊套件已於本次整理中移除；Git 歷史仍可供追溯。

## 狀態

2026-07-27 於美國（Comcast/Xfinity、SJC 出口）重新稽核。新增香香腐宅、動漫屋、Māngabz、漫畫櫃、熱辣漫畫與新版 zero搬運網；修復包子漫畫 App 宣傳假頁、包子漫畫2／G站共用 API、章節解碼與多 CDN 線路、香香腐宅新章節格式與 13 欄圖片重排、漫小肆／紳士漫畫新域名，以及所有 legacy 套件的大小寫相容圖示（包含漫畫狗）。G站不是封鎖亞洲以外訪客；它與包子漫畫2在全球抽測節點可讀，只有部分 ISP 的安全過濾仍可能攔截圖片網域。

目前發布 16 個通過建置和完整閱讀鏈測試的圖源；倉庫保留 32 個仍可能維護的來源，永久失效的 7 個來源及 74 個未被索引引用的舊套件／圖示已清除。候選來源也交叉檢查了 Aidoku Community 的 21 個中文來源與 Keiyoushi 的 51 個中文擴充；只有可匿名存取、可用 Aidoku 實作且圖片能在實際或分散式美國節點下載者才會加入。完整逐源結果、三部作品驗證、候選排除原因及相關 issue 請參閱 [AUDIT.md](AUDIT.md)。
