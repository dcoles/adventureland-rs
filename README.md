# AdventureLand.rs

A proof of concept showing how to use Rust code for [Adventure Land](https://adventure.land/).

## Building

```
wasm-pack build --target web --no-typescript
```

## Loader

```javascript
// Simple Loader
// ⚠️ IMPORTANT — Only use with URLs you control and trust
//
// Untrusted code may hijack your account, use it to mine crypto currency,
// steal personal data, cause your cat to run away and other nasty things.

(async () => {
  try {
    const { default: init } = await import('http://127.0.0.1:5500/adventureland_rs.js');
    init();
  } catch (e) {
    set_message('Load Error', 'red');
    safe_log(e.toString(), 'red')
  }
})();
```

## Serving

The generated `.js` and `.wasm` files need to be served via a HTTP server
to be usable. The easiest way is to use the provided small HTTP server
that only listens on your computer's local IP (`127.0.0.1`).

```
cd pkg
python ../code_server.py
```
