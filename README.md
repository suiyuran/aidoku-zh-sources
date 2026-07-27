# aidoku-zh-sources

繁體中文 | [简体中文](README.zh-CN.md)

Aidoku 中文圖源

## 如何使用

推薦使用 [v2 格式](https://aidoku.app/add-source-list/?url=https://raw.githubusercontent.com/NullKeeper-dev/aidoku-zh-sources/main/public/v2/index.min.json) 新增圖源，不會出現「legacy source list」警告。

舊版 Aidoku 仍可使用 [legacy 位址](https://aidoku.app/add-source-list/?url=https://raw.githubusercontent.com/NullKeeper-dev/aidoku-zh-sources/main/public/)；原有目錄位址和 `public/index.json` 均繼續保留。

如果無法存取 GitHub Raw，可以使用 [jsDelivr legacy 位址](https://aidoku.app/add-source-list/?url=https://cdn.jsdelivr.net/gh/NullKeeper-dev/aidoku-zh-sources@main/public/)。

## 圖源列表

- [18漫畫](https://18mh.org)
- [包子漫畫2](https://baozimh.org)
- [G站漫畫](https://godamh.com)
- [HO5HO](https://www.ho5ho.com)
- [Komiic漫畫](https://komiic.com)
- [Māngabz](https://mangabz.com)
- [漫畫櫃（Manhuagui）](https://www.manhuagui.com)
- [漫小肆韓漫](https://www.mxshm.top)
- [熱辣漫畫](https://www.manga2026.com)
- [紳士漫畫](https://www.wn07.cfd)
- [再漫畫](https://www.zaimanhua.com)

以上 11 個圖源都已完成「列表 → 詳情 → 章節 → 圖片」測試，且來源可見的章節均可匿名免費閱讀。熱辣漫畫網站本身另有付費區，但此來源固定使用 `free_type=1`，不會列出付費作品。

包子漫畫2與 G站使用兩站目前共用的 `v2.apikk.top` API，並依章節的 `line` 選擇圖片主機。封面解析會依序讀取 `data-src`、`data-original` 與 `src`，圖片請求也會附帶網站 Referer 與瀏覽器標頭。兩個來源的設定內可切換 Cloudflare、國際備用與舊版相容三條 CDN 線路。全球分散式測試的臺灣、新加坡、德國及多個美國節點都能讀圖；若 Comcast/Xfinity 將全部線路導向 SafeBrowse，需在 xFi Advanced Security 允許該網域或改用未攔截的網路，來源端無法繞過 ISP 過濾。

## 暫時下架

以下圖源仍保留原始碼，待網站恢復或完成較大改版後再重新發布：

- ~~[139漫畫](https://139mh.com)~~：JavaScript 防機器人頁
- ~~[巴卡漫畫](https://bakamh.com)~~：閱讀頁持續返回 HTTP 403
- ~~[嗶哩漫畫](https://www.bilimanga.net)~~：圖片 CDN 持續返回 HTTP 403
- ~~[拷貝漫畫](https://www.mangacopy.com)~~：新版章節 API 返回空圖片組
- ~~[漫畫狗](https://dogemanga.com)~~：美國線路搜尋／列表端點被 Cloudflare 返回 HTTP 403；舊套件的缺失圖示已修復
- ~~[古風漫畫](https://www.gufengmh.com)~~：網站返回 HTTP 403 / TLS 異常
- ~~[嗨皮漫畫](https://m.happymh.com)~~：持續出現人機驗證
- ~~[漫畫DB](https://www.manhuadb.com)~~：連線逾時或 JavaScript 挑戰
- ~~[MYCOMIC](https://mycomic.com/cn)~~：Cloudflare HTTP 403
- ~~[Nico](https://nicohub.cc/index?category=comic)~~：JavaScript 防機器人頁
- ~~[韓漫庫](https://se8.us)~~：首頁 HTTP 403，漫畫路由 HTTP 404
- ~~[野蠻漫畫](https://yemancomic.com)~~：美國線路持續返回 HTTP 403

## 已移除

以下網站已確認永久失效或不再提供原漫畫服務：無敵漫畫、動漫狂、Hanime1 漫畫區、ISEKAI、喵上漫畫、言耽社、舊版 zero搬運網（`zerobyw.github.io`）。

以下來源因列表包含登入、VIP、付費、鎖定或 App 專屬章節而移除：包子漫畫1、香香腐宅、動漫屋、嗶咔漫畫、漫畫人、漫客棧、NoyAcg、vomic漫畫、zero搬運網。其原始碼、套件與圖示均已刪除；Git 歷史仍可供追溯。

## 狀態

2026-07-27 於美國（Comcast/Xfinity、SJC 出口）重新稽核。包子漫畫2／G站已修復共用 API、章節解碼、圖片線路、lazy-load 封面屬性與圖片請求標頭；所有 Rust 來源亦重新檢查登入、VIP、付費、鎖定及 App 專屬章節。G站不是封鎖亞洲以外訪客；它與包子漫畫2在全球抽測節點可讀，只有部分 ISP 的安全過濾仍可能攔截圖片網域。

目前發布 11 個通過建置、完整閱讀鏈及免費性檢查的圖源；倉庫保留 23 個來源，其中 12 個因反爬或網站改版暫時下架。永久失效的 7 個來源、含存取限制的 9 個來源，以及未被索引引用的舊套件／圖示均已清除。完整逐源結果請參閱 [AUDIT.md](AUDIT.md)。
