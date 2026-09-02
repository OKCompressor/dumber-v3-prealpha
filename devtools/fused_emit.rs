// SPDX-License-Identifier: LicenseRef-Luna-Non-Commons-1.1
use std::{
    collections::BTreeMap,
    env,
    fs,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

fn chunk_index(path: &Path) -> Option<u32> {
    let s = path.file_name()?.to_string_lossy();
    let p = s.rfind("chunk_")?;
    let tail = &s[p + 6..];
    let digits: String = tail.chars()
        .take_while(|c| c.is_ascii_digit())
        .collect();

    if digits.is_empty() { None } else { digits.parse().ok() }
}

fn chunks(dir: &Path) -> Result<BTreeMap<u32, PathBuf>, String> {
    let mut out = BTreeMap::new();

    for e in fs::read_dir(dir).map_err(|e| e.to_string())? {
        let p = e.map_err(|e| e.to_string())?.path();

        if p.is_file() {
            if let Some(i) = chunk_index(&p) {
                out.insert(i, p);
            }
        }
    }

    Ok(out)
}

fn read_u24(path: &Path) -> Result<Vec<u32>, String> {
    let b = fs::read(path).map_err(|e| e.to_string())?;

    if b.len() % 3 != 0 {
        return Err(format!("bad u24 file {}", path.display()));
    }

    Ok(b.chunks_exact(3)
        .map(|x| {
            x[0] as u32
                | ((x[1] as u32) << 8)
                | ((x[2] as u32) << 16)
        })
        .collect())
}

fn read_u32(path: &Path) -> Result<Vec<u32>, String> {
    let b = fs::read(path).map_err(|e| e.to_string())?;

    if b.len() % 4 != 0 {
        return Err(format!("bad u32 file {}", path.display()));
    }

    Ok(b.chunks_exact(4)
        .map(|x| u32::from_le_bytes([x[0], x[1], x[2], x[3]]))
        .collect())
}

fn main() -> Result<(), String> {
    let a: Vec<String> = env::args().collect();

    if a.len() != 8 {
        return Err(format!(
            "usage: {} LOCAL_U16_DIR GMAP24_DIR R1_MAP_U32 MERGED_DICT SENTINEL_ID OUT_DIR EMIT_MAIN_0_OR_1",
            a[0]
        ));
    }

    let local_dir = Path::new(&a[1]);
    let gmap_dir = Path::new(&a[2]);
    let r1_map = read_u32(Path::new(&a[3]))?;
    let dict_bytes = fs::read(&a[4]).map_err(|e| e.to_string())?;
    let sentinel: u32 = a[5].parse::<u32>().map_err(|e| e.to_string())?;
    let out = Path::new(&a[6]);
    let emit_main = a[7] == "1";

    fs::create_dir_all(out).map_err(|e| e.to_string())?;

    let dict: Vec<&[u8]> = dict_bytes
        .split(|b| *b == b'\n')
        .filter(|x| !x.is_empty())
        .collect();

    let enc = chunks(local_dir)?;
    let maps = chunks(gmap_dir)?;

    if enc.keys().collect::<Vec<_>>() != maps.keys().collect::<Vec<_>>() {
        return Err("local-u16 and gmap chunk sets differ".into());
    }

    let mut total_tokens = 0u64;
    let mut total_singletons = 0u64;
    let mut total_lexical_bytes = 0u64;

    let mut chunk_summary = BufWriter::new(
        fs::File::create(out.join("CHUNKS.tsv")).map_err(|e| e.to_string())?
    );

    writeln!(
        chunk_summary,
        "chunk\ttokens\tsingletons\tfirst_r1\tlast_r1\tlexical_bytes"
    ).map_err(|e| e.to_string())?;

    for (idx, enc_path) in enc {
        let gm = read_u24(maps.get(&idx).unwrap())?;
        let b = fs::read(&enc_path).map_err(|e| e.to_string())?;

        if b.len() % 2 != 0 {
            return Err(format!("bad u16 stream {}", enc_path.display()));
        }

        let side_path = out.join(format!("chunk_{:03}.singletons.bin", idx));
        let mut side = BufWriter::new(
            fs::File::create(side_path).map_err(|e| e.to_string())?
        );

        let mut main = if emit_main {
            Some(BufWriter::new(
                fs::File::create(
                    out.join(format!("chunk_{:03}.r1.u32le", idx))
                ).map_err(|e| e.to_string())?
            ))
        } else {
            None
        };

        let mut tokens = 0u64;
        let mut singles = 0u64;
        let mut lexical_bytes = 0u64;
        let mut first_r1 = None;
        let mut last_r1 = None;

        for x in b.chunks_exact(2) {
            let local = u16::from_le_bytes([x[0], x[1]]) as usize;

            if local >= gm.len() {
                return Err(format!(
                    "chunk {} local={} map_len={}",
                    idx, local, gm.len()
                ));
            }

            let global = gm[local] as usize;

            if global >= r1_map.len() || global >= dict.len() {
                return Err(format!(
                    "global={} r1_map={} dict={}",
                    global,
                    r1_map.len(),
                    dict.len()
                ));
            }

            let r1 = r1_map[global];

            first_r1.get_or_insert(r1);
            last_r1 = Some(r1);

            if let Some(w) = main.as_mut() {
                w.write_all(&r1.to_le_bytes()).map_err(|e| e.to_string())?;
            }

            if r1 == sentinel {
                let tok = dict[global];

                side.write_all(&(tok.len() as u32).to_le_bytes())
                    .map_err(|e| e.to_string())?;
                side.write_all(tok).map_err(|e| e.to_string())?;

                singles += 1;
                lexical_bytes += tok.len() as u64;
            }

            tokens += 1;
        }

        side.flush().map_err(|e| e.to_string())?;

        if let Some(w) = main.as_mut() {
            w.flush().map_err(|e| e.to_string())?;
        }

        total_tokens += tokens;
        total_singletons += singles;
        total_lexical_bytes += lexical_bytes;

        writeln!(
            chunk_summary,
            "{}\t{}\t{}\t{}\t{}\t{}",
            idx,
            tokens,
            singles,
            first_r1.unwrap_or(0),
            last_r1.unwrap_or(0),
            lexical_bytes
        ).map_err(|e| e.to_string())?;
    }

    chunk_summary.flush().map_err(|e| e.to_string())?;

    let mut s = BufWriter::new(
        fs::File::create(out.join("SUMMARY.tsv")).map_err(|e| e.to_string())?
    );

    writeln!(s, "metric\tvalue").map_err(|e| e.to_string())?;
    writeln!(s, "tokens\t{}", total_tokens).map_err(|e| e.to_string())?;
    writeln!(s, "singleton_positions\t{}", total_singletons).map_err(|e| e.to_string())?;
    writeln!(s, "lexical_bytes\t{}", total_lexical_bytes).map_err(|e| e.to_string())?;
    writeln!(s, "sentinel_id\t{}", sentinel).map_err(|e| e.to_string())?;
    writeln!(s, "main_materialized\t{}", emit_main as u8).map_err(|e| e.to_string())?;

    println!("tokens={}", total_tokens);
    println!("singleton_positions={}", total_singletons);
    println!("lexical_bytes={}", total_lexical_bytes);
    println!("sentinel_id={}", sentinel);
    println!("main_materialized={}", emit_main as u8);

    Ok(())
}
