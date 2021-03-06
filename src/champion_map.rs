// url : http://ddragon.leagueoflegends.com/cdn/10.1.1/data/en_US/champion.json
// 2020.01.13 version 10.1.1
use phf::phf_map;

pub static CHAMPION_MAP: phf::Map<u32, &'static str> = phf_map! {
    1u32 => "Annie",
    2u32 => "Olaf",
    3u32 => "Galio",
    4u32 => "Twisted Fate",
    5u32 => "Xin Zhao",
    6u32 => "Urgot",
    7u32 => "LeBlanc",
    8u32 => "Vladimir",
    9u32 => "Fiddlesticks",
    10u32 => "Kayle",
    11u32 => "Master Yi",
    12u32 => "Alistar",
    13u32 => "Ryze",
    14u32 => "Sion",
    15u32 => "Sivir",
    16u32 => "Soraka",
    17u32 => "Teemo",
    18u32 => "Tristana",
    19u32 => "Warwick",
    20u32 => "Nunu & Willump",
    21u32 => "Miss Fortune",
    22u32 => "Ashe",
    23u32 => "Tryndamere",
    24u32 => "Jax",
    25u32 => "Morgana",
    26u32 => "Zilean",
    27u32 => "Singed",
    28u32 => "Evelynn",
    29u32 => "Twitch",
    30u32 => "Karthus",
    31u32 => "Cho'Gath",
    32u32 => "Amumu",
    33u32 => "Rammus",
    34u32 => "Anivia",
    35u32 => "Shaco",
    36u32 => "Dr. Mundo",
    37u32 => "Sona",
    38u32 => "Kassadin",
    39u32 => "Irelia",
    40u32 => "Janna",
    41u32 => "Gangplank",
    42u32 => "Corki",
    43u32 => "Karma",
    44u32 => "Taric",
    45u32 => "Veigar",
    48u32 => "Trundle",
    50u32 => "Swain",
    51u32 => "Caitlyn",
    53u32 => "Blitzcrank",
    54u32 => "Malphite",
    55u32 => "Katarina",
    56u32 => "Nocturne",
    57u32 => "Maokai",
    58u32 => "Renekton",
    59u32 => "Jarvan IV",
    60u32 => "Elise",
    61u32 => "Orianna",
    62u32 => "Wukong",
    63u32 => "Brand",
    64u32 => "Lee Sin",
    67u32 => "Vayne",
    68u32 => "Rumble",
    69u32 => "Cassiopeia",
    72u32 => "Skarner",
    74u32 => "Heimerdinger",
    75u32 => "Nasus",
    76u32 => "Nidalee",
    77u32 => "Udyr",
    78u32 => "Poppy",
    79u32 => "Gragas",
    80u32 => "Pantheon",
    81u32 => "Ezreal",
    82u32 => "Mordekaiser",
    83u32 => "Yorick",
    84u32 => "Akali",
    85u32 => "Kennen",
    86u32 => "Garen",
    89u32 => "Leona",
    90u32 => "Malzahar",
    91u32 => "Talon",
    92u32 => "Riven",
    96u32 => "Kog'Maw",
    98u32 => "Shen",
    99u32 => "Lux",
    101u32 => "Xerath",
    102u32 => "Shyvana",
    103u32 => "Ahri",
    104u32 => "Graves",
    105u32 => "Fizz",
    106u32 => "Volibear",
    107u32 => "Rengar",
    110u32 => "Varus",
    111u32 => "Nautilus",
    112u32 => "Viktor",
    113u32 => "Sejuani",
    114u32 => "Fiora",
    115u32 => "Ziggs",
    117u32 => "Lulu",
    119u32 => "Draven",
    120u32 => "Hecarim",
    121u32 => "Kha'Zix",
    122u32 => "Darius",
    126u32 => "Jayce",
    127u32 => "Lissandra",
    131u32 => "Diana",
    133u32 => "Quinn",
    134u32 => "Syndra",
    136u32 => "Aurelion Sol",
    141u32 => "Kayn",
    142u32 => "Zoe",
    143u32 => "Zyra",
    145u32 => "Kai'Sa",
    150u32 => "Gnar",
    154u32 => "Zac",
    157u32 => "Yasuo",
    161u32 => "Vel'Koz",
    163u32 => "Taliyah",
    164u32 => "Camille",
    201u32 => "Braum",
    202u32 => "Jhin",
    203u32 => "Kindred",
    222u32 => "Jinx",
    223u32 => "Tahm Kench",
    235u32 => "Senna",
    236u32 => "Lucian",
    238u32 => "Zed",
    240u32 => "Kled",
    245u32 => "Ekko",
    246u32 => "Qiyana",
    254u32 => "Vi",
    266u32 => "Aatrox",
    267u32 => "Nami",
    268u32 => "Azir",
    350u32 => "Yuumi",
    412u32 => "Thresh",
    420u32 => "Illaoi",
    421u32 => "Rek'Sai",
    427u32 => "Ivern",
    429u32 => "Kalista",
    432u32 => "Bard",
    497u32 => "Rakan",
    498u32 => "Xayah",
    516u32 => "Ornn",
    517u32 => "Sylas",
    518u32 => "Neeko",
    523u32 => "Aphelios",
    555u32 => "Pyke",
    875u32 => "Sett",
};
