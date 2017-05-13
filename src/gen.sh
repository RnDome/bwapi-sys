bindgen bridge.h -o bridge.rs \
  --no-unstable-rust \
  --opaque-type ".+_" \
  --blacklist-type "std.*|__.+|.+_$|Game_v(Send|Print|Draw).*|va_list|.+_t$" \
  --no-layout-tests \
  --no-derive-debug \
  --raw-line "#![allow(improper_ctypes)]" \
  -- -I../../bwapi-c/include

sed -i -r -- 's/.+\s+(.+)_;/pub struct \1;/' bridge.rs
