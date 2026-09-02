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
    let digits: String = tail.chars().take_while(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() { None } else { digits.parse().ok() }
}

fn collect_chunks(dir: &Path) -> Result<BTreeMap<u32, PathBuf>, String> {
    let mut out = BTreeMap::new();
    for e in fs::read_dir(dir).map_err(|e| e.to_string())? {
        let p = e.map_err(|e| e.to_string())?.path();
        if !p.is_file() {
            continue;
        }
        if let Some(i) = chunk_index(&p) {
            if out.insert(i, p.clone()).is_some() {
                return Err(format!("duplicate chunk index {}", i));
            }
        }
    }
    Ok(out)
}

fn read_u24(path: &Path) -> Result<Vec<u32>, String> {
    let b = fs::read(path).map_err(|e| e.to_string())?;
    if b.len() % 3 != 0 {
        return Err(format!("bad u24 length {}: {}", b.len(), path.display()));
    }
    Ok(b.chunks_exact(3)
        .map(|x| x[0] as u32 | ((x[1] as u32) << 8) | ((x[2] as u32) << 16))
        .collect())
}

fn main() -> Result<(), String> {
    let a: Vec<String> = env::args().collect();
    if a.len() != 4 {
        return Err(format!(
            "usage: {} LOCAL_U16_DIR GMAP24_DIR OUT_DIR",
            a[0]
        ));
    }

    let enc_dir = Path::new(&a[1]);
    let map_dir = Path::new(&a[2]);
    let out_dir = Path::new(&a[3]);

    fs::create_dir_all(out_dir).map_err(|e| e.to_string())?;

    let enc = collect_chunks(enc_dir)?;
    let maps = collect_chunks(map_dir)?;

    if enc.keys().collect::<Vec<_>>() != maps.keys().collect::<Vec<_>>() {
        return Err("encoded/gmap chunk indices differ".into());
    }

    let mut global_max = 0u32;

    for p in maps.values() {
        for g in read_u24(p)? {
            global_max = global_max.max(g);
        }
    }

    let mut counts = vec![0u64; global_max as usize + 1];
    let mut chunk_rows = Vec::new();
    let mut total_tokens = 0u64;

    for (idx, enc_path) in &enc {
        let map_path = maps.get(idx).unwrap();
        let map = read_u24(map_path)?;

        let b = fs::read(enc_path).map_err(|e| e.to_string())?;
        if b.len() % 2 != 0 {
            return Err(format!(
                "bad u16 length {}: {}",
                b.len(),
                enc_path.display()
            ));
        }

        let mut first: Option<u32> = None;
        let mut last: Option<u32> = None;
        let mut tokens = 0u64;

        for x in b.chunks_exact(2) {
            let local = u16::from_le_bytes([x[0], x[1]]) as usize;

            if local >= map.len() {
                return Err(format!(
                    "chunk {} local id {} >= map len {}",
                    idx,
                    local,
                    map.len()
                ));
            }

            let global = map[local];

            if first.is_none() {
                first = Some(global);
            }
            last = Some(global);

            counts[global as usize] += 1;
            tokens += 1;
        }

        total_tokens += tokens;

        chunk_rows.push((
            *idx,
            tokens,
            map.len(),
            first.unwrap_or(0),
            last.unwrap_or(0),
        ));

        println!(
            "chunk={:03} tokens={} local_vocab={} first_global={} last_global={}",
            idx,
            tokens,
            map.len(),
            first.unwrap_or(0),
            last.unwrap_or(0)
        );
    }

    let counts_path = out_dir.join("COUNTS.u64le");
    let mut w = BufWriter::new(
        fs::File::create(&counts_path).map_err(|e| e.to_string())?
    );

    for c in &counts {
        w.write_all(&c.to_le_bytes()).map_err(|e| e.to_string())?;
    }
    w.flush().map_err(|e| e.to_string())?;

    let mut cw = BufWriter::new(
        fs::File::create(out_dir.join("CHUNKS.tsv")).map_err(|e| e.to_string())?
    );

    writeln!(
        cw,
        "chunk\ttokens\tlocal_vocab\tfirst_global\tlast_global"
    ).map_err(|e| e.to_string())?;

    for (idx, tokens, vocab, first, last) in &chunk_rows {
        writeln!(
            cw,
            "{}\t{}\t{}\t{}\t{}",
            idx, tokens, vocab, first, last
        ).map_err(|e| e.to_string())?;
    }

    let nonzero = counts.iter().filter(|&&x| x > 0).count();
    let singleton_types = counts.iter().filter(|&&x| x == 1).count();

    let mut sw = BufWriter::new(
        fs::File::create(out_dir.join("SUMMARY.tsv")).map_err(|e| e.to_string())?
    );

    writeln!(sw, "metric\tvalue").map_err(|e| e.to_string())?;
    writeln!(sw, "chunks\t{}", chunk_rows.len()).map_err(|e| e.to_string())?;
    writeln!(sw, "tokens\t{}", total_tokens).map_err(|e| e.to_string())?;
    writeln!(sw, "global_vocab_slots\t{}", counts.len()).map_err(|e| e.to_string())?;
    writeln!(sw, "global_vocab_used\t{}", nonzero).map_err(|e| e.to_string())?;
    writeln!(sw, "singleton_types\t{}", singleton_types).map_err(|e| e.to_string())?;

    println!("tokens={}", total_tokens);
    println!("global_vocab_used={}", nonzero);
    println!("singleton_types={}", singleton_types);
    println!("counts={}", counts_path.display());

    Ok(())
}
