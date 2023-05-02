# Ultimate Cowardice

This is cowards version of [ultimate bravery](https://www.ultimate-bravery.net/)

The program to generates a random role and champion from a players champion pool.

## Usage

you need a folder that houses player files. The default is `players/`. This folder includes json files `$name.json`

each file looks like this 

```json
{
	"top":[...],
	"jng":[...],
	"mid":[...],
	"adc":[...],
	"sup":[...],
}
```

where `...` is a list of strings that correspond to champion names.

use the `-p` flag to specify players. They are seperated by `,`.

Example `ultimate-cowardice -p wunder,jankos,pekz,caps,mikyx`

run the program with -h for more info
