import { ArrayRef, Filter, Listing, ValueRef } from "aidoku-as/src";
import { Wdmh } from "./55dmh";

const source = new Wdmh();

export function get_manga_list(filter_list_descriptor: i32, page: i32): i32 {
  const filters: Filter[] = [];
  const objects = new ValueRef(filter_list_descriptor).asArray().toArray();
  for (let i = 0; i < objects.length; i++) {
    filters.push(new Filter(objects[i].asObject()));
  }
  return source.getMangaList(filters, page).value;
}

export function get_manga_listing(listing: i32, page: i32): i32 {
  return source.getMangaListing(new Listing(listing), page).value;
}

export function get_manga_details(manga_descriptor: i32): i32 {
  const id = new ValueRef(manga_descriptor).asObject().get("id").toString();
  return source.getMangaDetails(id).value;
}

export function get_chapter_list(manga_descriptor: i32): i32 {
  const id = new ValueRef(manga_descriptor).asObject().get("id").toString();
  const array = ArrayRef.new();
  const result = source.getChapterList(id);
  for (let i = 0; i < result.length; i++) {
    array.push(new ValueRef(result[i].value));
  }
  return array.value.rid;
}

export function get_page_list(chapter_descriptor: i32): i32 {
  const id = new ValueRef(chapter_descriptor).asObject().get("id").toString();
  const array = ArrayRef.new();
  const result = source.getPageList(id);
  for (let i = 0; i < result.length; i++) {
    array.push(new ValueRef(result[i].value));
  }
  return array.value.rid;
}
