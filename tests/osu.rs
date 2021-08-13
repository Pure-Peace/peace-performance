#![cfg(feature = "osu")]

extern crate peace_performance;

use peace_performance::{Beatmap, osu::DifficultyAttributes};

struct MapResult {
    map_id: u32,
    mods: u32,
    stars: f32,
    pp: f32,
}

fn margin() -> f32 {
    if cfg!(feature = "no_sliders_no_leniency") {
        0.0075
    } else if cfg!(feature = "no_leniency") {
        0.0025
    } else if cfg!(feature = "all_included") {
        0.001
    } else {
        unreachable!()
    }
}

fn osu_test(map: Beatmap, result: &MapResult) {
    let margin = margin();

    let star_margin = margin;
    let pp_margin = margin;

    let MapResult {
        map_id,
        mods,
        stars,
        pp,
    } = result;

    let mut osupp =  peace_performance::OsuPP::new(&map).mods(*mods).accuracy(99.59);
    let result = osupp.calculate();
    let attr = osupp.attributes.unwrap();
    println!("{} {} {}", attr.max_combo, attr.n_circles, attr.n_spinners);
    

    println!("{} {} {} {} {}", result.stars(), star_margin, stars, map_id, mods);
    println!("{} {} {} {} {}", result.pp(), pp_margin, pp, map_id, mods);
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
                    match tokio::fs::File::open(format!("./maps/{}.osu", result.map_id)).await {
                        Ok(file) => file,
                        Err(why) => panic!("Could not open file {}.osu: {}", result.map_id, why),
                    };

                let map = match Beatmap::parse(file).await {
                    Ok(map) => map,
                    Err(why) => panic!("Error while parsing map {}: {}", result.map_id, why),
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
        map_id: 1851299,
        mods: 200,
        stars: 4.19951953364192,
        pp: 95.35544846090738,
    }, 
    MapResult {
        map_id: 51651,
        mods: 200,
        stars: 4.19951953364192,
        pp: 95.35544846090738,
    },
    MapResult {
        map_id: 888,
        mods: 200,
        stars: 4.19951953364192,
        pp: 95.35544846090738,
    }, 
    MapResult {
        map_id: 666,
        mods: 200,
        stars: 4.19951953364192,
        pp: 95.35544846090738,
    },
    MapResult {
        map_id: 9999,
        mods: 200,
        stars: 4.19951953364192,
        pp: 95.35544846090738,
    }, 
    MapResult {
        map_id: 3333,
        mods: 200,
        stars: 4.19951953364192,
        pp: 95.35544846090738,
    },
    MapResult {
        map_id: 9988,
        mods: 200,
        stars: 4.19951953364192,
        pp: 95.35544846090738,
    }
    
];
