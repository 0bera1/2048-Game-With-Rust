# 2048 — Rust + WebAssembly (WASM)

Rust ile yazılmış ve WebAssembly olarak tarayıcıda çalışan 2048 oyunudur. Grafikler `Canvas 2D` ile çizilir, oyun mantığı Rust tarafında çalışır ve `wasm-bindgen` ile JavaScript'e bağlanır.

## Özellikler

- Rust ile performanslı oyun mantığı
- WebAssembly çıktısı ve `pkg/` klasöründe hazır JS bağlayıcıları
- `Canvas 2D` ile basit ve akıcı görseller
- Klavye kontrolleri: Ok tuşları veya WASD; yeniden başlat: R

---

## Hızlı Başlangıç (Derlemeden Çalıştır)

Bu depo, `pkg/` klasöründe derlenmiş dosyalarla gelir. Yalnızca statik bir dosya sunucusu ile çalıştırabilirsiniz.

1) Depoyu indirin/klonlayın
```bash
git clone https://github.com/0bera1/2048-Game-With-Rust/
cd 2048
```

2) Basit bir statik sunucu başlatın (tercihlerden birini seçin)
```bash
# Python
python3 -m http.server 8000

# veya Node.js (npx)
npx serve . -p 8000 --single

# veya Rust ekosistemi
cargo install basic-http-server
basic-http-server . -a 127.0.0.1:8000
```

3) Tarayıcıda açın
```
http://localhost:8000
```

Sayfa açıldığında oyun otomatik başlar. `index.html`, `pkg/game_2048.js` modülünü yükleyerek `start("game")` çağırır.

> Not: `file://` üzerinden açarsanız CORS/Module hataları alırsınız; mutlaka bir HTTP sunucusu kullanın.

---

## Geliştirici Kurulumu (Kaynaktan Derleme)

Projeyi kendiniz derlemek veya değişiklikleri görmek için aşağıdaki adımları izleyin.

### Gereksinimler

- Rust ve Cargo (rustup ile)
- WASM hedefi: `wasm32-unknown-unknown`
- Aşağıdaki araçlardan biri:
  - wasm-pack (önerilen) veya
  - wasm-bindgen-cli

Hedefi ekleyin:
```bash
rustup target add wasm32-unknown-unknown
```

### Yöntem 1: wasm-pack ile

wasm-pack kurun (bir kez):
```bash
cargo install wasm-pack
```

Derleyin ve çıktıyı `pkg/` içine alın:
```bash
wasm-pack build --target web --out-dir pkg --release
```

Sunucu başlatın ve `index.html`'i açın (bkz. Hızlı Başlangıç 2. ve 3. adım).

### Yöntem 2: wasm-bindgen-cli ile

Önce derleyin:
```bash
cargo build --release --target wasm32-unknown-unknown
```

wasm-bindgen-cli kurun (bir kez):
```bash
cargo install wasm-bindgen-cli
```

WASM bağlayıcılarını üretin:
```bash
wasm-bindgen \
  --target web \
  --out-dir pkg \
  target/wasm32-unknown-unknown/release/game_2048.wasm
```

Ardından bir statik sunucu ile `index.html`'i açın.

> Geliştirme modu için `--release` olmadan derleyebilir veya wasm-pack'te `--dev` kullanabilirsiniz.

---

## Kullanım

- Ok tuşları veya WASD ile taşları hareket ettirin.
- R ile oyunu sıfırlayın.
- Oyun bittiğinde veya kazandığınızda skor üstte gösterilir.

---

## Proje Yapısı (Kısa)

- `src/domain/`: Oyun kuralları, yönler, tahta ve hareket olayları
- `src/application/`: `GameService` ile oyun akışı ve skor yönetimi
- `src/infra/`: `Canvas2DRenderer` ve `wasm_bindings` ile tarayıcı entegrasyonu
- `index.html`: Tarayıcı giriş noktası; `pkg/` çıktısını yükler

---

## Sorun Giderme

- "Failed to load module" / CORS hatası: `file://` yerine bir HTTP sunucusu kullanın.
- `wasm-pack` veya `wasm-bindgen` bulunamadı: İlgili aracı `cargo install` ile kurun.
- Derleme hataları: `rustup target add wasm32-unknown-unknown` komutunu çalıştırdığınızdan emin olun.

---

## Lisans

Bu proje eğitim ve demo amaçlıdır. Dilediğiniz gibi inceleyin ve geliştirin.
