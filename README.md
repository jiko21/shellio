# shellio
[![pr-check](https://github.com/jiko21/shellio/actions/workflows/pr-check.yml/badge.svg)](https://github.com/jiko21/shellio/actions/workflows/pr-check.yml)
[![release](https://github.com/jiko21/shellio/actions/workflows/release.yml/badge.svg)](https://github.com/jiko21/shellio/actions/workflows/release.yml)

shell test tools for cli apps

## overview
shellio allows you to test cli tools with snapshot.

Only you have to do is write spec file `shellio.spec.json` and run command `shellio`, then snapshot of standard io is saved in .spec dir.

## about shellio.spec.json
You can write test case in shellio.spec.json.

```json
{
  "name": "sample", // the name of whole tests.
  "specs": [ // you can write test cases here
    {
      "describe": "sample1", // test case description
      "command": "ls", // command to exec for test
      "results": ".spec/sample1_ls.snapshot" // path for snapshot
    }
  ]
}
```
At first, you should make `results` blank. (shellio will fill in `results` later)
