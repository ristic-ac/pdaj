# Paralelne i distribuirane arhitekture i jezici

Ovaj repozitorijum sadrži materijal, primere koda i pomoćne resurse za
kurs **Paralelne i distribuirane arhitekture i jezici**, sa fokusom na
programski jezik **Rust** i njegovo korišćenje za paralelizaciju algoritama.

## 🛠️ Alati

- **Kompajler:** `rustc` (v1.75.0 se koristi u laboratoriji)
- **Upravljač projektom i zavisnostima:** `cargo`  (v1.75.0 se koristi u laboratoriji)
- **Ekstenzije za razvojno okruženje:** Rust Analyzer

## 📅 Plan rada po sedmicama

| Datum          | V   | Tema                                                                                     |
| -------------- | --- | ---------------------------------------------------------------------------------------- |
| **05.11.2025** |     | Nema održavanja vežbi                                                                    |
| **19.11.2025** | 1   | Uvod, upravljanje projektom uz pomoć `cargo`-a                                           |
| **26.11.2025** | 2   | Osnovni koncepti jezika Rust                                                             |
| **03.12.2025** | 3   | Vlasništvo (Ownership)                                                                   |
| **10.12.2025** | 4   | Strukture, generički tipovi i enumeracije                                                |
| **24.12.2025** | 5   | Životni vekovi (Lifetimes), osobine (Traits), obrada grešaka                             |
| **26.12.2025** | 6   | Pametni pokazivači, closures, iteratori                                                  |
| **31.12.2025** | 7   | Sistemsko programiranje                                                                  |
| **14.12.2025** | 8   | Upravljanje datotekama, SerDe, argumenti komandne linije, kolekcije, pattern matching    |
| **21.01.2026** |     | Priprema za kolokvijum                                                                   |
| **24.01.2026** |     | Kolokvijum                                                                               |
| **28.01.2026** | 9   | Multithreading, `async/await`                                                            |
| **31.01.2026** |     | TBD (Konsultacije)                                                                       |
| **04.02.2026** |     | TBD (Konsultacije)                                                                       |
| **11.02.2026** |     | TBD (Konsultacije)                                                                       |

## 📚 Sadržaj repozitorijuma

- Primeri koda po nedeljama
- Demonstracije jezika Rust
- Vežbe iz paralelizma i konkurentnosti
- Zadaci za pripremu za kolokvijum (_TODO_)
- Prezentacije sa vežbi (_TODO_)

## 🚀 Kako početi

1. Instalirati Rust alatke: <https://www.rust-lang.org/tools/install>

2. Klonirati repozitorijum:

   ```bash
   git clone <url-repozitorijuma>
   ```

3. Pokrenuti primer:

   ```bash
   cargo run
   ```

## 📖 Lokalna dokumentacija

Za rad u laboratorijskom okruženju koristi se specifična verzija jezika (1.75.0). Dokumentaciju je moguće instalirati lokalno radi pristupa bez internet veze.

### Instalacija

Pokrenuti sledeće komande u terminalu:

```bash
VER=1.75.0
rustup toolchain install "$VER" --component rust-docs
rustup component add rust-docs --toolchain "$VER"
rustup +"$VER" doc
```

Nakon instalacije, HTML dokumentacija se nalazi na sledećoj putanji:

`~/.rustup/toolchains/1.75.0-<arch>/share/doc/rust/html/index.html`

Primer za x86_64 Linux: `~/.rustup/toolchains/1.75.0-x86_64-unknown-linux-gnu/share/doc/rust/html/index.html`

## 🤝 Doprinos

Slobodno otvorite **issue** ili **pull request** za predloge
unapređenja, bugfixeve ili dodatne primere.
Studenti su ohrabreni da doprinose svojim rešenjima, i biće nagrađeni u vidu dodatnih bodova.

## 🎓 Priznanje izvora (Attribution)

Neki primeri i objašnjenja u ovom repozitorijumu prilagođeni su iz
**Rust Book (Brown University edition)**, dostupnog na <https://rust-book.cs.brown.edu/>.

Originalni sadržaj Rust Book-a je pod dvojnim licenciranjem **MIT OR Apache-2.0**,
a svi izvedeni delovi u ovom repozitorijumu zadržavaju iste uslove licenciranja.

## 📄 Licenca

Ovaj projekat je licenciran pod jednom od sledećih licenci, po izboru:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

Delovi ovog rada zasnovani su na Rust Book-u (Brown University edition),
koji je takođe licenciran pod uslovima **MIT OR Apache-2.0**.
