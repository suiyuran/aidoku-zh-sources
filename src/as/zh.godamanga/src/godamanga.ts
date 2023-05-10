import {
  Chapter,
  Filter,
  FilterType,
  Html,
  HttpMethod,
  Listing,
  Manga,
  MangaContentRating,
  MangaPageResult,
  MangaStatus,
  MangaViewer,
  Page,
  Request,
  Source,
} from "aidoku-as/src";

const FILTER_GENRE = [
  "",
  "cn",
  "kr",
  "jp",
  "lianai",
  "xuanhuan",
  "danvzhu",
  "rexue",
  "xuanyi",
  "gufeng",
  "chuanyue",
  "doushi",
  "gaoxiao",
  "mengxi",
  "xitong",
  "wuxia",
  "shaonv",
  "xiaoyuan",
  "kongbu",
  "zhiyu",
];

export class Godamanga extends Source {
  genExploreURL(type: string, genre: string, page: number): string {
    let url = `https://godamanga.com/`;
    url += genre === "" ? `${type}/` : `manga-${genre.length === 2 ? "genre" : "tag"}/${genre}/`;
    url += page > 1 ? `page/${page as i32}/` : "";
    return url;
  }

  genSearchURL(query: string): string {
    return `https://godamanga.com?s=${encodeURI(query)}`;
  }

  transMangaStatus(status: string): MangaStatus {
    if (status === "连载中") return MangaStatus.Ongoing;
    if (status === "已完结") return MangaStatus.Completed;
    if (status === "已弃坑") return MangaStatus.Cancelled;
    if (status === "已暂停") return MangaStatus.Hiatus;
    return MangaStatus.Unknown;
  }

  getImageURL(image: Html): string {
    return image.attr(image.attr("class").includes("lazyload") ? "data-src" : "src");
  }

  handlerChapterTitle(title: string): string {
    const words = title.split(" ");
    words.pop();
    if (title.endsWith("ago")) words.pop();
    return words.join(" ");
  }

  getHTML(url: string): Html {
    const request = Request.create(HttpMethod.GET);
    request.url = url;
    return request.html();
  }

  getMangaList(filters: Filter[], page: number): MangaPageResult {
    let query = "";
    let genre = "";

    for (let i = 0; i < filters.length; i++) {
      const filter = filters[i];

      if (filter.type === FilterType.Title) {
        query = filter.value.toString();
      }
      if (filter.type === FilterType.Select) {
        const index = filter.value.toInteger() as i32;
        if (filter.name === "类型") {
          genre = FILTER_GENRE[index];
        }
      }
    }

    const notSearch = query === "";
    const url = notSearch ? this.genExploreURL("manga", genre, page) : this.genSearchURL(query);
    const html = this.getHTML(url);
    const list = html.select(".entries>article").array();
    const hasMore = notSearch;
    const mangas: Manga[] = [];

    for (let i = 0; i < list.length; i++) {
      const item = list[i];
      const id = item
        .select("a")
        .attr("href")
        .split("/")
        .filter((a: string) => !!a)
        .pop();
      const title = item.select("h2>a").text().trim();
      const manga = new Manga(id, title);
      manga.cover_url = this.getImageURL(item.select("a>img"));
      mangas.push(manga);
    }

    return new MangaPageResult(mangas, hasMore);
  }

  getMangaListing(listing: Listing, page: number): MangaPageResult {
    let type = "";

    if (listing.name === "最新上架") {
      type = "newss";
    }
    if (listing.name === "人气推荐") {
      type = "hots";
    }
    if (listing.name === "热门更新") {
      type = "dayup";
    }

    const url = this.genExploreURL(type, "", page);
    const html = this.getHTML(url);
    const list = html.select(".entries>article").array();
    const hasMore = true;
    const mangas: Manga[] = [];

    for (let i = 0; i < list.length; i++) {
      const item = list[i];
      const id = item
        .select("a")
        .attr("href")
        .split("/")
        .filter((a: string) => !!a)
        .pop();
      const title = item.select("h2>a").text().trim();
      const manga = new Manga(id, title);
      manga.cover_url = this.getImageURL(item.select("a>img"));
      mangas.push(manga);
    }

    return new MangaPageResult(mangas, hasMore);
  }

  getMangaDetails(mangaId: string): Manga {
    const url = `https://godamanga.com/manga/${mangaId}`;
    const html = this.getHTML(url);
    const id = mangaId;
    const title = html.select("h1").text().trim();
    const manga = new Manga(id, title);
    manga.cover_url = html.select("meta[property='og:image']").attr("content");
    manga.author = html
      .select(".author-content>a")
      .array()
      .map((a: Html) => a.text().trim())
      .join(", ");
    manga.artist = "";
    manga.description = html.select("meta[property='og:description']").attr("content").trim();
    manga.url = url;
    manga.categories = html
      .select(".genres-content>a")
      .array()
      .map((a: Html) => a.text().replace("热门漫画", "").replace("玄幻n穿越", "").trim())
      .filter((a: string) => !!a);
    const status = html.select(".genres-content+.author-content").text().replace("状态：", "").trim();
    manga.status = this.transMangaStatus(status);
    manga.rating = MangaContentRating.Safe;
    manga.viewer = manga.categories.includes("日漫") ? MangaViewer.RTL : MangaViewer.Scroll;
    return manga;
  }

  getChapterList(mangaId: string): Chapter[] {
    const url = `https://godamanga.com/chapterlist/${mangaId}`;
    const html = this.getHTML(url);
    const list = html.select("ul>a").array();
    const last = list.length - 1;
    const chapters: Chapter[] = [];

    for (let i = last; i >= 0; i--) {
      const item = list[last - i];
      const id = item.attr("id").replace("_", "/");
      const title = this.handlerChapterTitle(item.text().trim());
      const chapter = new Chapter(id, title);
      chapter.chapter = (i + 1) as f32;
      chapter.url = `https://godamanga.com/manga/${id}`;
      chapters.push(chapter);
    }

    return chapters;
  }

  getPageList(chapterId: string): Page[] {
    const url = `https://godamanga.com/manga/${chapterId}`;
    const html = this.getHTML(url);
    const list = html.select(".stk-block-content>img").array();
    const pages: Page[] = [];

    for (let i = 0; i < list.length; i++) {
      const item = list[i];
      const page = new Page(i);
      page.url = this.getImageURL(item);
      pages.push(page);
    }

    return pages;
  }
}
