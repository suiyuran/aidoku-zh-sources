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

export class Dogemanga extends Source {
  genExploreURL(set: string): string {
    return `https://dogemanga.com?s=${set}`;
  }

  genSearchURL(query: string, page: number): string {
    return `https://dogemanga.com?q=${encodeURI(query)}&o=${((page - 1) * 24) as i32}`;
  }

  transMangaStatus(status: string): MangaStatus {
    if (status.includes("連載完結")) return MangaStatus.Completed;
    if (status.includes("連載中")) return MangaStatus.Ongoing;
    return MangaStatus.Unknown;
  }

  getHTML(url: string): Html {
    const request = Request.create(HttpMethod.GET);
    request.url = url;
    return request.html();
  }

  getMangaList(filters: Filter[], page: number): MangaPageResult {
    let query = "";

    for (let i = 0; i < filters.length; i++) {
      const filter = filters[i];

      if (filter.type === FilterType.Title) {
        query = filter.value.toString();
      }
    }

    const url = query === "" ? this.genExploreURL("") : this.genSearchURL(query, page);
    const html = this.getHTML(url);
    const list = html.select("div[data-manga-id]").array();
    const hasMore = query !== "";
    const mangas: Manga[] = [];

    for (let i = 0; i < list.length; i++) {
      const item = list[i];
      const id = item.attr("data-manga-id");
      const title = item.select(".site-card__manga-title").text().trim();
      const manga = new Manga(id, title);
      manga.cover_url = item.select(".card-img-top").attr("src");
      mangas.push(manga);
    }

    return new MangaPageResult(mangas, hasMore);
  }

  getMangaListing(listing: Listing, page: number): MangaPageResult {
    let set = "";

    if (listing.name === "最新连载") {
      set = "1";
    }

    const url = this.genExploreURL(set);
    const html = this.getHTML(url);
    const list = html.select("div[data-manga-id]").array();
    const hasMore = false;
    const mangas: Manga[] = [];

    for (let i = 0; i < list.length; i++) {
      const item = list[i];
      const id = item.attr("data-manga-id");
      const title = item.select(".site-card__manga-title").text().trim();
      const manga = new Manga(id, title);
      manga.cover_url = item.select(".card-img-top").attr("src");
      mangas.push(manga);
    }

    return new MangaPageResult(mangas, hasMore);
  }

  getMangaDetails(mangaId: string): Manga {
    const url = `https://dogemanga.com/m/${mangaId}`;
    const html = this.getHTML(url);
    const title = html.select(".site-card__manga-title").text().trim();
    const manga = new Manga(mangaId, title);
    manga.cover_url = html.select("site-manga__cover-image").attr("src");
    manga.author = html
      .select("h4>.site-card__link")
      .array()
      .map((a: Html) => a.text().trim())
      .join(", ");
    manga.artist = "";
    manga.description = html.select(".site-card__brief").text().trim();
    manga.url = url;
    manga.categories = [];
    manga.status = this.transMangaStatus(html.select("p>small").text());
    manga.rating = MangaContentRating.Safe;
    manga.viewer = MangaViewer.RTL;
    return manga;
  }

  getChapterList(mangaId: string): Chapter[] {
    const url = `https://dogemanga.com/m/${mangaId}`;
    const html = this.getHTML(url);
    const list = html.select("option[value]").array();
    const last = list.length - 1;
    const chapters: Chapter[] = [];

    for (let i = last; i >= 0; i--) {
      const item = list[last - i];
      const id = item.attr("value").split("/").pop();
      const title = item.text().trim();
      const chapter = new Chapter(id, title);
      chapter.chapter = (i + 1) as f32;
      chapter.url = `https://dogemanga.com/p/${id}`;
      chapters.push(chapter);
    }

    return chapters;
  }

  getPageList(chapterId: string): Page[] {
    const url = `https://dogemanga.com/p/${chapterId}`;
    const html = this.getHTML(url);
    const list = html.select("img[data-page-id]").array();
    const pages: Page[] = [];

    for (let i = 0; i < list.length; i++) {
      const item = list[i];
      const page = new Page(i as i32);
      page.url = item.attr("data-page-image-url");
      pages.push(page);
    }

    return pages;
  }
}
