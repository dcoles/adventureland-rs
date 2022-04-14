# AdventureLand.rs

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
