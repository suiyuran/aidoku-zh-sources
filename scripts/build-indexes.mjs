import fs from "node:fs";
import path from "node:path";

const [v2Path, legacyPath, legacyMinPath] = process.argv.slice(2);

if (!v2Path || !legacyPath || !legacyMinPath) {
  throw new Error(
    "Usage: node scripts/build-indexes.mjs <v2-index.min.json> <legacy-index.json> <legacy-index.min.json>",
  );
}

const v2Text = fs.readFileSync(v2Path, "utf8");
const v2 = JSON.parse(v2Text);

if (
  !v2 ||
  Array.isArray(v2) ||
  typeof v2.name !== "string" ||
  !Array.isArray(v2.sources)
) {
  throw new Error("The Aidoku CLI output is not a v2 source-list object.");
}

const requiredFields = [
  "id",
  "name",
  "version",
  "iconURL",
  "downloadURL",
  "languages",
  "contentRating",
];

for (const source of v2.sources) {
  for (const field of requiredFields) {
    if (!(field in source)) {
      throw new Error(`v2 source ${source.id ?? "(unknown)"} is missing ${field}.`);
    }
  }
  if (
    !Array.isArray(source.languages) ||
    source.languages.length === 0 ||
    !Number.isInteger(source.contentRating)
  ) {
    throw new Error(`v2 source ${source.id} has invalid language/rating metadata.`);
  }
}

const legacy = v2.sources.map((source) => ({
  id: source.id,
  name: source.name,
  file: path.posix.basename(source.downloadURL),
  icon: path.posix.basename(source.iconURL),
  lang: source.languages[0],
  version: source.version,
  nsfw: source.contentRating,
}));

fs.writeFileSync(legacyPath, `${JSON.stringify(legacy, null, 2)}\n`);
fs.writeFileSync(legacyMinPath, JSON.stringify(legacy));

// The compact v2 output must remain byte-for-byte JSON.stringify-compatible,
// matching serde_json::to_writer in the official Aidoku CLI.
if (v2Text !== JSON.stringify(v2)) {
  throw new Error("The v2 index is not in the canonical compact form.");
}
