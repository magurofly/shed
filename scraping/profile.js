// AtCoder のプロフィールページを取得して大まかなデータを返す
// @return { country, birthYear, twitter, topcoder, codeforces, affiliation, rank, rating, highest, count, last }
async function fetchAtCoderProfile(user) {
  const text = await fetch(`https://atcoder.jp/users/${user}?lang=en`).then(res => res.text());
  const doc = new DOMParser().parseFromString(text, "text/html");
  const results = {};
  const mapping = {
    "Birth Year": "birthYear",
    "Twitter ID": "twitter",
    "TopCoder ID": "topcoder",
    "Codeforces ID": "codeforces",
    "Affiliation": "affiliation",
    "Rated Matches ": "count",
    "Last Competed": "last",
  };
  for (const table of doc.querySelectorAll("table.dl-table")) for (const row of table.rows) {
    const name = row.cells[0].textContent;
    switch (name) {
      case "Country/Region":
        results.country = row.querySelector("img").src.match(/\w+(?=\.png)/)[0];
        break;
      case "Rank":
        results.rank = row.cells[1].textContent.match(/\d+/)[0];
        break;
      case "Rating":
        results.rating = row.cells[1].textContent.match(/\d+/)[0];
        break;
      case "Highest Rating":
        results.highest = row.cells[1].children[0].textContent;
        break;
      default:
        if (name in mapping) results[mapping[name]] = row.cells[1].textContent;
    }
  }
  return results;
}
