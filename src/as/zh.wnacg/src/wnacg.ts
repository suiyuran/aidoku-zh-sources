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

export class Wnacg extends Source {
  genExploreURL(cate: string, page: number): string {
    return `https://www.wnacg.com/albums-index-page-${page as i32}-cate-${cate}.html`;
  }

  genSearchURL(query: string, page: number): string {
    return `https://www.wnacg.com/search/index.php?q=${encodeURI(query)}&s=create_time_DESC&p=${page as i32}`;
  }

  getHTML(url: string): Html {
    const request = Request.create(HttpMethod.GET);
    request.url = url;
    return request.html();
  }

  getStr(url: string): string {
    const request = Request.create(HttpMethod.GET);
    request.url = url;
    return request.string();
  }

  getMangaList(filters: Filter[], page: number): MangaPageResult {
    let query = "";

    for (let i = 0; i < filters.length; i++) {
      const filter = filters[i];

      if (filter.type === FilterType.Title) {
        query = filter.value.toString();
      }
    }

    const url = query === "" ? this.genExploreURL("", page) : this.genSearchURL(query, page);
    const html = this.getHTML(url);
    const list = html.select("ul>li").array();
    const hasMore = true;
    const mangas: Manga[] = [];

    for (let i = 0; i < list.length; i++) {
      const item = list[i];
      const id = item.select(".pic_box>a").attr("href").split("-").pop().replace(".html", "");
      const title = item.select(".info>.title>a").text().trim();
      const manga = new Manga(id, title);
      manga.cover_url = "https:" + item.select(".pic_box>a>img").attr("src");
      mangas.push(manga);
    }

    return new MangaPageResult(mangas, hasMore);
  }

  getMangaListing(listing: Listing, page: number): MangaPageResult {
    let cate = "";

    if (listing.name === "同人志") {
      cate = "1";
    }
    if (listing.name === "单行本") {
      cate = "9";
    }
    if (listing.name === "杂志&短篇") {
      cate = "10";
    }
    if (listing.name === "韩漫") {
      cate = "20";
    }

    const url = this.genExploreURL(cate, page);
    const html = this.getHTML(url);
    const list = html.select("ul>li").array();
    const mangas: Manga[] = [];
    const hasMore = true;

    for (let i = 0; i < list.length; i++) {
      const item = list[i];
      const id = item.select(".pic_box>a").attr("href").split("-").pop().replace(".html", "");
      const title = item.select(".info>.title>a").text().trim();
      const manga = new Manga(id, title);
      manga.cover_url = "https:" + item.select(".pic_box>a>img").attr("src");
      mangas.push(manga);
    }

    return new MangaPageResult(mangas, hasMore);
  }

  getMangaDetails(mangaId: string): Manga {
    const url = `https://www.wnacg.com/photos-index-aid-${mangaId}.html`;
    const html = this.getHTML(url);
    const title = html.select("#bodywrap>h2").text();
    const manga = new Manga(mangaId, title);
    manga.cover_url = html.select("#bodywrap>div>.uwthumb>img").attr("src").replace("//", "https:");
    manga.author = "";
    manga.artist = "";
    manga.description = "";
    manga.url = url;
    manga.categories = html
      .select("#bodywrap>div>.uwconn>label:nth-child(1)")
      .text()
      .replace("分類：", "")
      .split("／")
      .map((a: string) => a.split("&"))
      .flat()
      .map((a: string) => a.trim());
    manga.status = MangaStatus.Unknown;
    manga.rating = MangaContentRating.NSFW;
    manga.viewer = MangaViewer.RTL;
    return manga;
  }

  getChapterList(mangaId: string): Chapter[] {
    const url = `https://www.wnacg.com/photos-index-aid-${mangaId}.html`;
    const chapter = new Chapter(mangaId, "第 1 话");
    chapter.chapter = 1 as f32;
    chapter.url = url;
    return [chapter];
  }

  getPageList(chapterId: string): Page[] {
    const url = `https://www.wnacg.com/photos-gallery-aid-${chapterId}.html`;
    const str = this.getStr(url);
    const urls = str.split('\\"').filter((a: string) => a.startsWith("//"));
    const pages: Page[] = [];

    for (let i = 0; i < urls.length; i++) {
      const page = new Page(i);
      page.url = "https:" + urls[i];
      pages.push(page);
    }

    return pages;
  }
}
