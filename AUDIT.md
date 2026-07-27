# 圖源稽核記錄

稽核日期：2026-07-27
線上測試位置：美國 Comcast/Xfinity（Cloudflare SJC 出口）

本次枚舉 `src/as/` 與 `src/rust/` 下全部 39 個圖源。建置欄表示原始碼可產生 `.aix`；線上欄依序檢查列表、漫畫詳情、章節列表、章節頁及至少一張實際圖片。發佈標準是匿名使用且整條閱讀鏈能由美國網路直接完成；需要登入、持續 Cloudflare 挑戰、圖片 CDN 被 ISP 攔截或只剩 App 宣傳圖者不進入 v2 索引。

「全球可用」無法由單一測試地點絕對保證，因此本表採較保守的標準：在臺灣曾可用、但美國鏈路失敗的圖源也先下架，並保留原始碼供日後重新測試。

## 彙總

| 狀態 | 數量 | 說明 |
| --- | ---: | --- |
| 發佈 | 14 | 建置成功，並通過美國完整閱讀鏈 |
| 暫時下架 | 18 | 反爬、認證、API 改版或區域／ISP 圖片鏈路問題 |
| 已移除 | 7 | 域名失效、停站或原漫畫服務已不存在 |
| 合計 | 39 | 全部原始碼皆可建置 |

## 逐源結果

| 圖源 | 基礎 URL | 建置 | 美國線上檢查 | 失敗類型／備註 | 相關 issue |
| --- | --- | :---: | --- | --- | --- |
| `zh.139mh` 139漫畫 | <https://139mh.com> | ✅ | ❌ 列表失敗 | HTTP 200，但內容只有 JavaScript「Loading...」防機器人頁 | — |
| `zh.18mh` 18漫畫 | <https://18mh.org> | ✅ | ✅ 完整鏈路 | 列表、詳情、42 章、40 頁及原圖 HTTP 200 | — |
| `zh.55dmh` 無敵漫畫 | <https://www.55dmh.com> | ✅ | ❌ 首頁失敗 | 連線被拒，站點已停止服務 | — |
| `zh.bakamh` 巴卡漫畫 | <https://bakamh.com> | ✅ | ❌ 章節頁失敗 | 列表需預熱 Cookie／Client Hints；閱讀路由仍持續 HTTP 403 | [#124](https://github.com/suiyuran/aidoku-zh-sources/issues/124)、[#148](https://github.com/suiyuran/aidoku-zh-sources/issues/148) |
| `zh.baozimh1` 包子漫畫1 | <https://www.baozimh.com> | ✅ | ✅ 3 部作品完整鏈路 | 使用目前 `twmanga.com` 閱讀頁；略過網站替換的 App 宣傳假頁，較早可讀章節原圖 HTTP 200 | [#138](https://github.com/suiyuran/aidoku-zh-sources/issues/138)、[Aidoku #485](https://github.com/Skittyblock/aidoku-community-sources/issues/485)、[Suwayomi #686](https://github.com/Suwayomi/Suwayomi-Server/issues/686) |
| `zh.baozimh2` 包子漫畫2 | <https://baozimh.org> | ✅ | ❌ 圖片失敗 | API／章節資料可解碼，但與 G站共用的圖片 CDN 在 Comcast 線路發生 TLS／SafeBrowse 攔截 | [#138](https://github.com/suiyuran/aidoku-zh-sources/issues/138) |
| `zh.bilicomic` 嗶哩漫畫 | <https://www.bilimanga.net> | ✅ | ❌ 圖片失敗 | 列表、詳情及章節可解析；`i.motiezw.com` 圖片持續 HTTP 403 | — |
| `zh.boylove` 香香腐宅 | <https://boylove.cc> | ✅ | ✅ 完整鏈路 | 修復 JSON／HTML Accept 協商、失效章節 API、內嵌 `imageData` 與 13 欄圖片重排；原圖 HTTP 200 | — |
| `zh.cartoonmad` 動漫狂 | <https://www.cartoonmad.com> | ✅ | ❌ 首頁失敗 | 所有路徑只返回 `/lander` 停放頁跳轉 | — |
| `zh.copymanga` 拷貝漫畫 | <https://www.mangacopy.com> | ✅ | ❌ 章節圖片失敗 | 切換目前參數與 `ccz` 解密後，多部作品仍返回空圖片組 | [#128](https://github.com/suiyuran/aidoku-zh-sources/issues/128)、[#135](https://github.com/suiyuran/aidoku-zh-sources/issues/135) |
| `zh.dm5` 動漫屋 | <https://www.dm5.com> | ✅ | ✅ 完整鏈路 | 多部作品的列表、詳情、章節與圖片均通過官方 Aidoku 測試執行器 | — |
| `zh.dogemanga` 漫畫狗 | <https://dogemanga.com> | ✅ | ❌ 列表失敗 | 首頁可開，但美國 `_search` 端點被 Cloudflare 返回 HTTP 403；legacy 套件圖示已補齊 | — |
| `zh.godamanga` G站漫畫 | <https://godamh.com> | ✅ | ❌ 圖片失敗 | 首頁與目前 API 可開；漫畫圖片 CDN 在部分美國 ISP 路徑 TLS 失敗，詳見專項結果 | [#142](https://github.com/suiyuran/aidoku-zh-sources/issues/142)、[#144](https://github.com/suiyuran/aidoku-zh-sources/issues/144) |
| `zh.gufengmh` 古風漫畫 | <https://www.gufengmh.com> | ✅ | ❌ 首頁失敗 | HTTPS/TLS 異常；HTTP 返回 nginx 403 | — |
| `zh.hanime1` Hanime1.me | <https://hanime1.me/comics> | ✅ | ❌ 漫畫服務失效 | 漫畫路由在行動版 HTTP 404、桌面版 Cloudflare 403；主站只剩影片服務 | [#147](https://github.com/suiyuran/aidoku-zh-sources/issues/147) |
| `zh.happymh` 嗨皮漫畫 | <https://m.happymh.com> | ✅ | ❌ 首頁／API 失敗 | HTTP 403 人機驗證；圖片算法也改為新的加密掃描格式 | — |
| `zh.ho5ho` HO5HO | <https://www.ho5ho.com> | ✅ | ✅ 完整鏈路 | 目前卡片、詳情與閱讀頁選擇器正常；原圖 HTTP 200 | — |
| `zh.isekai` ISEKAI | <https://isekai.ch> | ✅ | ❌ 首頁失敗 | 所有路徑均為域名停放頁 | — |
| `zh.komiic` Komiic漫畫 | <https://komiic.com> | ✅ | ✅ 完整鏈路 | GraphQL 列表、詳情、章節及圖片均成功 | [#127](https://github.com/suiyuran/aidoku-zh-sources/issues/127) |
| `zh.mangabz` Māngabz | <https://mangabz.com> | ✅ | ✅ 完整鏈路 | 列表、詳情、章節腳本及圖片通過官方 Aidoku 測試執行器 | — |
| `zh.manhuabika` 嗶咔漫畫 | <https://manhuabika.com> | ✅ | ⚠️ 無法匿名驗收 | 必須使用有效帳號登入；本次不建立帳號或繞過認證 | — |
| `zh.manhuadb` 漫畫DB | <https://www.manhuadb.com> | ✅ | ❌ 首頁失敗 | HTTPS 逾時；HTTP 返回混淆 JavaScript 挑戰 | — |
| `zh.manhuagui` 漫畫櫃 | <https://www.manhuagui.com> | ✅ | ✅ 完整鏈路 | 列表、詳情、章節解碼及圖片通過官方 Aidoku 測試執行器 | — |
| `zh.manhuaren` 漫畫人 | <https://www.manhuaren.com> | ✅ | ❌ API 認證失敗 | 舊 token／簽章失效；目前需要匿名裝置註冊、RSA 與新簽名流程 | — |
| `zh.miaoshangmanhua` 喵上漫畫 | <https://www.miaoshangmanhua.cc> | ✅ | ❌ DNS 失敗 | 域名無法解析 | — |
| `zh.mkzhan` 漫客棧 | <https://www.mkzhan.com> | ✅ | ✅ 完整鏈路 | 列表 API、詳情、章節、頁面及圖片均成功 | — |
| `zh.mxshm`（ID `zh.msxhm`）漫小肆韓漫 | <https://www.mxshm.top> | ✅ | ✅ 完整鏈路 | 更新目前域名；將舊圖片主機的 `/static/` 路徑改寫回可用主站，原圖 HTTP 200 | — |
| `zh.mycomic` MYCOMIC | <https://mycomic.com/cn> | ✅ | ❌ 首頁失敗 | Cloudflare HTTP 403 | — |
| `zh.nicohub` Nico | <https://nicohub.cc/index?category=comic> | ✅ | ❌ 首頁失敗 | HTTP 200，但只有 JavaScript「Loading...」挑戰頁 | — |
| `zh.noy1` NoyAcg | <https://noy1.top> | ✅ | ❌ API 認證失敗 | 舊 API 與 beta API 均返回 `status: login` | [#151](https://github.com/suiyuran/aidoku-zh-sources/issues/151) |
| `zh.relamanhua` 熱辣漫畫 | <https://www.manga2026.com> | ✅ | ✅ 完整鏈路 | 修復作者／標籤／狀態選擇器、大小寫資料屬性及圖片預設值；原圖 HTTP 200 | — |
| `zh.se8` 韓漫庫 | <https://se8.us> | ✅ | ❌ 首頁／路由失敗 | 首頁 nginx 403；舊漫畫路由及 `index.php` HTTP 404 | — |
| `zh.vomicmh` vomic漫畫 | <http://www.vomicmh.com> | ✅ | ❌ API 失敗 | 網站已重寫為新前端，舊 API HTTP 404 | — |
| `zh.wnacg` 紳士漫畫 | <https://www.wn07.cfd> | ✅ | ✅ 完整鏈路 | 更新到目前可用域名；列表、詳情、188 頁章節及圖片 HTTP 200 | [#149](https://github.com/suiyuran/aidoku-zh-sources/issues/149)、[#152](https://github.com/suiyuran/aidoku-zh-sources/issues/152) |
| `zh.yandanshe` 言耽社 | <https://yandanshe.com> | ✅ | ❌ DNS 失敗 | 域名無法解析 | — |
| `zh.yemancomic` 野蠻漫畫 | <https://yemancomic.com> | ✅ | ❌ 首頁失敗 | 臺灣曾可讀；目前美國線路持續 HTTP 403，因此不符合發佈標準 | — |
| `zh.zaimanhua` 再漫畫 | <https://www.zaimanhua.com> | ✅ | ✅ 完整鏈路 | 列表、詳情、章節頁及圖片 HTTP 200 | — |
| `zh.zerobyw` zero搬運網 | <https://www.zerobyw33.com> | ✅ | ✅ 完整鏈路 | 新版來源登入初始化、列表、詳情、章節與圖片通過官方 Aidoku 測試執行器 | — |
| `zh.zerobywns` 舊版 zero搬運網 | <https://zerobyw.github.io> | ✅ | ❌ 服務失效 | GitHub Pages 只剩說明頁；舊內容域名已變成無關 WordPress，插件 API 404 | [#145](https://github.com/suiyuran/aidoku-zh-sources/issues/145) |

## G站／包子漫畫2：美國無法讀圖的原因

這不是單純的「亞洲以外 IP 一律封鎖」：

- `godamh.com` 首頁和目前 `v2.apikk.top` 漫畫／章節 API 從美國可正常返回。
- 真正失敗的是舊 `api-get-v3.mgsearcher.com`、`g-mh.online` 及目前 `*.6wm.top` 圖片主機；在本次 Comcast/Xfinity 路徑會出現 `ERR_SSL_PACKET_LENGTH_TOO_LONG`、TLS frame 錯誤或 SafeBrowse 攔截。
- 因此會呈現「網站能開、書也看得到，但圖片載不出來」。同一組後端也影響 `baozimh.org`。
- 改 DNS、Referer、User-Agent 或直接使用 API 都無法修復 ISP 到圖片 CDN 的傳輸路徑。把流量轉送到第三方代理會增加隱私、成本與單點故障，本倉庫不會暗中加入代理。

所以 `zh.godamanga` 與 `zh.baozimh2` 暫時標記 `deprecated`，不進入索引。這也代表原先要求的 G站三部作品驗收無法在「全球直接連線」條件下誠實通過；[#142](https://github.com/suiyuran/aidoku-zh-sources/issues/142) 與 [#144](https://github.com/suiyuran/aidoku-zh-sources/issues/144) 保留為已診斷但未能全球修復。

## 包子漫畫1：三部作品驗證

2023 年其他閱讀器所報告的問題是客戶端會偵測並裁掉插入圖片內的 banner。2026-07-27 的實際回應已不同：部分最新章節的六張圖片全部都是「請在包子漫畫 App 閱讀」宣傳圖，完全沒有漫畫像素可供裁切還原；較早章節則直接提供乾淨原圖。來源現在會辨識並略過固定 App 宣傳路徑，不再把廣告當漫畫頁。

| 測試標題 | 詳情／章節列表 | 抽測可讀章節 | 頁數 | 圖片結果 |
| --- | --- | ---: | ---: | --- |
| 武煉巔峰 | HTTP 200，超過 3,800 章 | slot 3800 | 13 | HTTP 200，282,802 bytes |
| カコミスル老師四格合集 | HTTP 200，2,735 個章節連結 | slot 2600 | 2 | HTTP 200，254,528 bytes |
| 中國驚奇先生 | HTTP 200，1,853 個章節連結 | slot 1700 | 48 | HTTP 200，542,207 bytes |

三部作品都通過列表、詳情、章節列表、可讀章節頁與原圖下載。App 專屬的新章仍然不能由網頁來源取得，README 已明確標示此限制。

## 香香腐宅圖片修復

舊來源使用的 `xxblapingpong.cc` 章節 API 已無法連線。現行網站改為在章節 HTML 內提供 `imageData`，並以 `randomClass = 13` 把高度低於 4,000 px 的圖片切成直欄逆序排列。來源已：

1. 避免在 HTML 請求的 `Accept` 內宣告 `application/json`，防止伺服器把整頁 HTML 包成 JSON 字串。
2. 直接解析現行章節頁的 `imageData`。
3. 使用 Aidoku Canvas 按網站演算法還原 13 欄圖片；高圖維持原樣。
4. 加入正確 Referer、語言與瀏覽器標頭。

官方 `aidoku-test-runner` 已成功下載《那對貴族兄弟的事》章節圖片（HTTP 200）；同一測試亦驗證列表、詳情及章節列表。

## 線上候選來源範圍

本次不是只搜尋幾個站名，而是交叉比對兩個仍活躍的公開擴充目錄：

- [Aidoku-Community/sources](https://github.com/Aidoku-Community/sources)：檢查全部 21 個 `zh.*` Aidoku 原生來源。新增並實測通過 `zh.boylove`、`zh.dm5`、`zh.mangabz`、`zh.manhuagui`、`zh.relamanhua`、`zh.zerobyw`；授權文字已放入 `LICENSES/`。
- [Keiyoushi extensions-source](https://github.com/keiyoushi/extensions-source)：檢查 `src/zh/` 的 51 個中文擴充作為站點發現清單。

沒有直接搬入的候選主要分成：

- 已在本倉庫存在：包子漫畫、巴卡、嗶哩、拷貝、G站、嗨皮、Komiic、NoyAcg、紳士漫畫、再漫畫等。
- 必須登入或付費：嗶咔、JMComic、部分官方／商業平台。
- 美國匿名鏈路失敗：Cloudflare 403、Turnstile、空圖片 API 或圖片 CDN TLS 錯誤。
- `zh.rumanhua`：列表、詳情與章節可解析，但實際 `dmw.546457.xyz` 圖片在美國發生 TLS corrupted frame，因此未發佈。
- 只有 Kotlin／其他閱讀器實作、尚無可驗證 Aidoku 版本者不會被假裝成已完成來源。

「所有可能來源」在本次交付中的可驗收定義，是上述目錄中能匿名使用、具合法可移植實作且能從美國完成整條 Aidoku 閱讀鏈的來源；無法讀圖的候選不會為了增加數量而放進索引。

## 索引與套件相容性

- `public/v2/index.min.json`：Aidoku 目前 v2 物件格式，結構為 `name` + `sources`，由目前官方 CLI 產生 canonical minified 輸出。
- `public/v2/index.json`：同一 v2 索引的可讀版本。
- `public/index.json`：保留 legacy 陣列格式。
- `public/index.min.json`：保留 legacy 壓縮陣列；舊目錄型訂閱會讀取此檔，不可用 v2 物件覆蓋。
- 兩套索引目前都列出 14 個發佈來源；legacy 舊版 `.aix` 仍保留，避免既有下載網址失效。
- 所有發佈包與保留的 legacy 包都同時包含 `Payload/icon.png`、`Payload/Icon.png`。漫畫狗舊包已確認補上兩個路徑，不再顯示空白圖示。

本倉庫同時含 Aidoku 舊版 WASM Source API 與新版 Source API；這和「來源清單 v2 schema」是兩個層級。新來源使用目前 `aidoku-rs`，舊來源仍由 Aidoku 的 legacy runner 相容載入。
