#![cfg(feature = "osu")]

extern crate peace_performance;

use peace_performance::{Beatmap};

struct MapResult<'a> {
    mapname: &'a str,
    mods: u32
}

fn osu_test(map: Beatmap, result: &MapResult) {
    
    let MapResult {
        mapname: _,
        mods
    } = result;

    let mut osupp =  peace_performance::OsuPP::new(&map).mods(*mods).accuracy(100.0);
    let ppresult = osupp.calculate();
    let attributes = osupp.attributes.unwrap();
    let mut slider_bonus = 1.0;
    let slider_total_combo = attributes.max_combo - attributes.n_circles - attributes.n_spinners;
    let slider_combo_percentage = (slider_total_combo as f32) / (attributes.max_combo as f32);
    let combo_per_slider = slider_total_combo as f32 / osupp.map.n_sliders as f32;
    if slider_combo_percentage > 0.5 && combo_per_slider < 2.1 { 
        slider_bonus += ((slider_combo_percentage * 100.0 - 50.0).powf(0.3) * (1.5 / ((combo_per_slider - 2.0) * 10.0)).powf(0.5)) / 10.0 * 1.1
    }
    println!("{}", result.mapname);
    println!("slider_combo_percentage: {} combo_per_slider: {}", slider_combo_percentage, combo_per_slider);
    println!("PP: {}, bouns: {}", ppresult.pp, slider_bonus);
}

#[cfg(not(any(feature = "async_std", feature = "async_tokio")))]
#[test]
fn osu_sync() {
    for result in RESULTS {
        let file = match std::fs::File::open(format!("./maps/{}.osu", result.map_id)) {
            Ok(file) => file,
            Err(why) => panic!("Could not open file {}.osu: {}", result.map_id, why),
        };

        let map = match Beatmap::parse(file) {
            Ok(map) => map,
            Err(why) => panic!("Error while parsing map {}: {}", result.map_id, why),
        };

        osu_test(map, result);
    }
}

#[cfg(feature = "async_tokio")]
#[test]
fn osu_async_tokio() {
    tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("could not start runtime")
        .block_on(async {
            for result in RESULTS {
                let file =
                    match tokio::fs::File::open(format!("./maps/{}.osu", result.mapname)).await {
                        Ok(file) => file,
                        Err(why) => panic!("Could not open file {}.osu: {}", result.mapname, why),
                    };

                let map = match Beatmap::parse(file).await {
                    Ok(map) => map,
                    Err(why) => panic!("Error while parsing map {}: {}", result.mapname, why),
                };

                osu_test(map, result);
            }
        });
}

#[cfg(feature = "async_std")]
#[test]
fn osu_async_std() {
    async_std::task::block_on(async {
        for result in RESULTS {
            let file =
                match async_std::fs::File::open(format!("./maps/{}.osu", result.map_id)).await {
                    Ok(file) => file,
                    Err(why) => panic!("Could not open file {}.osu: {}", result.map_id, why),
                };

            let map = match Beatmap::parse(file).await {
                Ok(map) => map,
                Err(why) => panic!("Error while parsing map {}: {}", result.map_id, why),
            };

            osu_test(map, result);
        }
    })
}

const RESULTS: &[MapResult] = &[
    MapResult {
        mapname: "Wakeshima Kanon - Tsukinami (Reform) [Nostalgia]",
        mods: 200
    },
    MapResult {
        mapname: "Kano - Daisy Blue (Rieri) [Hope]",
        mods: 200
    },
    MapResult {
        mapname: "KiRaRe - 367Days (_kotachi_) [Over the Dreams]",
        mods: 200
    },
    MapResult {
        mapname: "HoneyWorks - Akatsuki Zukuyo ([C u r i]) [Taeyang's Extra]",
        mods: 200
    },
    MapResult {
        mapname: "HoneyWorks - Miraizu feat.Aida Miou(CVToyosaki Aki) (Fycho) [Special]",
        mods: 200
    },
    MapResult {
        mapname: "FROZEN QUALIA - Aisubeki Hibi e (Beomsan) [Tsumia's Extra]",
        mods: 200
    },
    MapResult {
        mapname: "Yunomi - Wakusei Rabbit (feat. TORIENA) (Cellina) [Usagi]",
        mods: 200
    },
    MapResult {
        mapname: "Hatsuki Yura - Yoiyami Hanabi (Lan wings) [Lan]",
        mods: 200
    },
    MapResult {
        mapname: "Shoji Meguro - Kimi no Kioku (Aethral Remix) (Akali) [Remembrance]",
        mods: 200
    },
    MapResult {
        mapname: "Oomori Seiko - JUSTadICE (TV Size) (fieryrage) [Extreme]",
        mods: 200
    },
    MapResult {
        mapname: "Kano - Sakura no Zenya (Woood13) [Tears]",
        mods: 200
    },
    MapResult {
        mapname: "MOMOIRO CLOVER Z - SANTA SAN (A r M i N) [1-2-SANTA]",
        mods: 200
    },
    MapResult {
        mapname: "LiSA - Jet Rocket (Wen) [Rocketing Love]",
        mods: 200
    },
    MapResult {
        mapname: "seiya-murai feat. ALT - Sumidagawa Karenka (Nevo) [Remembrance]",
        mods: 200
    },
    MapResult {
        mapname: "fhana - where you are (Sotarks) [Melancholy]",
        mods: 200
    },
    MapResult {
        mapname: "Poppin'Party x Aya (CV Maeshima Ami) x Kokoro (CV Itou Miku) - Geki! Teikoku Kagekidan (Left) [Left x Karen x bbj0920's Expert]",
        mods: 200
    },
    MapResult {
        mapname: "Yunomi - Koi no Uta (feat. Yuzaki Tsukasa (CV Kito Akari)) (TV Size) (hypercyte) [Expert]",
        mods: 200
    },
    MapResult {
        mapname: "fhana - Hoshikuzu no Interlude (Sotarks) [Melancholy]",
        mods: 200
    },
    MapResult {
        mapname: "fhana - Anemone no Hana (Sotarks) [Melancholy]",
        mods: 200
    },
    MapResult {
        mapname: "765 MILLION ALLSTARS - UNION!! (Fu3ya_) [WE ARE ALL MILLION!!]",
        mods: 200
    },
    MapResult {
        mapname: "Camellia feat. Nanahira - Bassdrop Freaks (Long Ver.) (RLC) [Extra]",
        mods: 128
    },
    MapResult {
        mapname: "the peggies - Kimi no Sei (TV Size) (Sotarks) [Extra]",
        mods: 200
    }  
];
